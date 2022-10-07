package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"sort"
	"strconv"
	"strings"
	"time"

	"github.com/go-redis/redis/v8"

	dapr "github.com/dapr/go-sdk/client"
)

const (
	stateStoreName = `statestore`
	countKey       = `count`
)

func storeCount(api string, old_size int, new_size int, start_time time.Time) {
	daprClient, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	//defer daprClient.Close()
	ctx := context.Background()

	var key = "image-api-" + api
	fmt.Printf("key: %s", key)

	old_image_size := float64(old_size) / 1024
	process_image_size := float64(new_size) / 1024
	cost_time := time.Since(start_time).Seconds()

	store_msg := fmt.Sprintf("old_size: %.2f (kb), cur_size: %.2f (kb) cost_time: %.3f (s)",
		old_image_size, process_image_size, cost_time)
	count, _ := daprClient.GetState(ctx, stateStoreName, countKey, nil)
	curCount, _ := strconv.ParseInt(string(count.Value), 10, 32)
	curCount++

	state_err := daprClient.SaveState(ctx, stateStoreName, countKey, []byte(strconv.FormatInt(curCount, 10)), nil)
	if state_err != nil {
		fmt.Printf("Failed to persist state: %v\n", state_err)
	} else {
		fmt.Printf("Successfully persisted state\n")
	}

	eventKey := "event-" + strconv.FormatInt(curCount, 10)
	timestamp := time.Now().Format("2006-01-02 15:04:05")
	eventVal := fmt.Sprintf("%s,%s,%s", timestamp, key, store_msg)
	state_err = daprClient.SaveState(ctx, stateStoreName, eventKey, []byte(eventVal), nil)
	if state_err != nil {
		fmt.Printf("Failed to persist state: %v\n", state_err)
	} else {
		fmt.Printf("Successfully persisted state\n")
	}
}

func daprClientSend(image []byte, w http.ResponseWriter, api string) {
	// create the client
	client, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	//defer client.Close()
	ctx := context.Background()

	content := &dapr.DataContent{
		ContentType: "text/plain",
		Data:        image,
	}

	start_time := time.Now()
	resp, err := client.InvokeMethodWithContent(ctx, "image-api-go", "/api/image", "post", content)
	if err != nil {
		panic(err)
	}
	storeCount(api, len(image), len(resp), start_time)

	log.Printf("dapr-wasmedge-go method api/image has invoked, response: %d", len(resp))
	//fmt.Printf("Image result: %q\n", resp)
	w.Header().Set("Content-Type", "image/png")
	fmt.Fprintf(w, "%s", string(resp))
}

func httpClientSend(image []byte, w http.ResponseWriter, api string) {
	client := &http.Client{}
	fmt.Printf("httpClientSend ....", api)

	// Dapr api format: http://localhost:<daprPort>/v1.0/invoke/<appId>/method/<method-name>
	var uri string
	var response_type string
	if api == "classify" {
		uri = "http://localhost:3504/v1.0/invoke/image-api-classify/method/classify"
		response_type = "text/plain"
	} else {
		uri = "http://localhost:3503/v1.0/invoke/image-api-grayscale/method/grayscale"
		response_type = "image/png"
	}
	println("uri: ", uri)

	start_time := time.Now()
	req, err := http.NewRequest("POST", uri, bytes.NewBuffer(image))

	if err != nil {
		panic(err)
	}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	println(resp.Header.Get("Content-Type"))
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	storeCount(api, len(image), len(body), start_time)

	res := string(body)
	//println("res: ", res)
	if strings.Contains(res, "Max bytes limit exceeded") {
		res = "ImageTooLarge"
	}

	w.Header().Set("Content-Type", response_type)
	fmt.Fprintf(w, "%s", res)
}

func imageHandler(w http.ResponseWriter, r *http.Request) {
	body, err := ioutil.ReadAll(r.Body)

	if err != nil {
		println("error: ", err.Error())
		panic(err)
	}
	api := r.Header.Get("api")
	println("imageHandler .... : ", api)
	if api == "go" {
		daprClientSend(body, w, api)
	} else {
		httpClientSend(body, w, api)
	}
}

func Max(x, y int64) int64 {
	if x < y {
		return y
	}
	return x
}

func statHandler(w http.ResponseWriter, r *http.Request) {
	println("stateHandler ....")
	daprClient, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	//defer daprClient.Close()
	ctx := context.Background()

	if err != nil {
		println("error: ", err.Error())
		panic(err)
	}
	count, _ := daprClient.GetState(ctx, stateStoreName, countKey, nil)
	curCount, _ := strconv.ParseInt(string(count.Value), 10, 32)
	// get the last 10 events
	res := make(map[string]string)
	for i := curCount; i > Max(0, curCount-int64(10)); i-- {
		eventKey := "event-" + strconv.FormatInt(i, 10)
		eventVal, _ := daprClient.GetState(ctx, stateStoreName, eventKey, nil)
		res[eventKey] = string(eventVal.Value)
	}
	keys := make([]string, 0, len(res))
	for k := range res {
		keys = append(keys, k)
	}
	sort.Sort(sort.Reverse(sort.StringSlice(keys)))
	resp := make([]string, 0, len(res))
	for _, k := range keys {
		resp = append(resp, k+"##"+res[k])
	}
	jsonResp, _ := json.Marshal(resp)
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, "%s", jsonResp)
}

func homepageHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "text/html")
	content, _ := ioutil.ReadFile("./static/home.html")
	fmt.Print("homepageHandler ....")
	ctx := context.Background()
	client, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	option_str := ""
	for i := 0; i <= 1; i++ {
		key := "option-" + strconv.Itoa(i)
		items, err := client.GetConfigurationItem(ctx, "dapr-wasm-config", key)
		if err != nil {
			panic(err)
		}
		option_key := string((*items).Value)
		option_val := strings.ReplaceAll(strings.ToLower(option_key), " ", "-")
		option_str += fmt.Sprintf("\n<option value=\"%s\">%s</option>", option_val, option_key)
	}
	content = bytes.Replace(content, []byte("{options}"), []byte(option_str), 1)
	if content == nil {
		w.WriteHeader(http.StatusNotFound)
	} else {
		fmt.Fprintf(w, "%s", content)
	}
}

func init() {
	opts := &redis.Options{
		Addr: "127.0.0.1:6379",
	}
	client := redis.NewClient(opts)
	// set config value
	client.Set(context.Background(), "option-0", "Grayscale", -1)
	client.Set(context.Background(), "option-1", "Classify", -1)
}

func main() {
	http.HandleFunc("/static/home.html", homepageHandler)
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/static/", http.StripPrefix("/static/", fs))
	http.HandleFunc("/api/hello", imageHandler)
	http.HandleFunc("/api/invokecount", statHandler)
	println("listen to 8080 ...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
