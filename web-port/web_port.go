package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strings"
	"time"

	dapr "github.com/dapr/go-sdk/client"
)

const (
	stateStoreName = `statestore`
)

func storeCount(api string, msg string) {
	daprClient, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	//defer daprClient.Close()
	ctx := context.Background()

	var key = "image-api-" + api
	fmt.Printf("key: %s", key)
	fmt.Printf("msg: %s", msg)
	state_err := daprClient.SaveState(ctx, stateStoreName, key, []byte(msg))
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

	image_size := len(image)
	resp, err := client.InvokeMethodWithContent(ctx, "image-api-go", "/api/image", "post", content)
	if err != nil {
		panic(err)
	}

	storeCount(api, fmt.Sprintf("image_size: %d", image_size))

	log.Printf("dapr-wasmedge-go method api/image has invoked, response: %s", string(resp))
	fmt.Printf("Image classify result: %q\n", resp)
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "%s", string(resp))
}

func httpClientSend(image []byte, w http.ResponseWriter, api string) {
	client := &http.Client{}
	fmt.Printf("httpClientSend ....")

	// Dapr api format: http://localhost:<daprPort>/v1.0/invoke/<appId>/method/<method-name>
	var uri string
	if api == "rust" {
		uri = "http://localhost:3502/v1.0/invoke/image-api-rs/method/api/image"
	} else {
		uri = "http://localhost:3503/v1.0/invoke/image-api-wasi-socket-rs/method/image"
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
	end_time := time.Now()

	//println(resp)
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	old_image_size := float64(len(image)) / 1024
	process_image_size := float64(len(body)) / 1024
	cost_time := end_time.Sub(start_time).Seconds()
	store_msg := fmt.Sprintf("old_size: %.2f (kb), cur_size: %.2f (kb) cost_time: %.3f (s)",
		old_image_size, process_image_size, cost_time)
	storeCount(api, store_msg)
	fmt.Printf("herrrrrrrrr")

	res := string(body)
	//println("res: ", res)
	if strings.Contains(res, "Max bytes limit exceeded") {
		res = "ImageTooLarge"
	}
	fmt.Printf("finished process !!!!!!!!!!!!")
	w.Header().Set("Content-Type", "image/png")
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

	api := r.URL.Query().Get("api")
	curStr, state_err := daprClient.GetState(ctx, stateStoreName, "image-api-"+api)
	if state_err != nil {
		fmt.Printf("Failed to persist state: %v\n", state_err)
	}

	resp := make(map[string]string)
	resp["count"] = string(curStr.Value)
	jsonResp, _ := json.Marshal(resp)
	w.Header().Set("Content-Type", "application/json")
	fmt.Fprintf(w, "%s", jsonResp)
}

func main() {
	fs := http.FileServer(http.Dir("./static"))
	http.Handle("/static/", http.StripPrefix("/static/", fs))
	http.HandleFunc("/api/hello", imageHandler)
	http.HandleFunc("/api/invokecount", statHandler)
	println("listen to 8080 ...")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
