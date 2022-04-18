package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"strconv"
	"strings"

	dapr "github.com/dapr/go-sdk/client"
)

const (
	stateStoreName = `statestore`
)

func storeCount(api string) {

	daprClient, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}
	ctx := context.Background()

	var key = "image-api-" + api
	curStr, state_err := daprClient.GetState(ctx, stateStoreName, key)
	if state_err != nil {
		fmt.Printf("Failed to persist state: %v\n", state_err)
	}
	curCount, _ := strconv.ParseInt(string(curStr.Value), 10, 32)
	curCount++

	println("key: ", key)
	println("curCount: ", curCount)
	state_err = daprClient.SaveState(ctx, stateStoreName, key, []byte(strconv.Itoa(int(curCount))))
	if state_err != nil {
		fmt.Printf("Failed to persist state: %v\n", state_err)
	} else {
		fmt.Printf("Successfully persisted state\n")
	}
}

func daprClientSend(image []byte, w http.ResponseWriter, api string) {
	ctx := context.Background()

	// create the client
	client, err := dapr.NewClient()
	if err != nil {
		panic(err)
	}

	content := &dapr.DataContent{
		ContentType: "text/plain",
		Data:        image,
	}

	resp, err := client.InvokeMethodWithContent(ctx, "image-api-go", "/api/image", "post", content)
	if err != nil {
		panic(err)
	}

	storeCount(api)

	log.Printf("dapr-wasmedge-go method api/image has invoked, response: %s", string(resp))
	fmt.Printf("Image classify result: %q\n", resp)
	w.WriteHeader(http.StatusOK)
	fmt.Fprintf(w, "%s", string(resp))
}

func httpClientSend(image []byte, w http.ResponseWriter, api string) {
	client := &http.Client{}
	println("httpClientSend ....")

	// Dapr api format: http://localhost:<daprPort>/v1.0/invoke/<appId>/method/<method-name>
	var uri string
	if api == "rust" {
		uri = "http://localhost:3502/v1.0/invoke/image-api-rs/method/api/image"
	} else {
		uri = "http://localhost:3503/v1.0/invoke/image-api-wasi-socket-rs/method/image"
	}
	println("uri: ", uri)

	req, err := http.NewRequest("POST", uri, bytes.NewBuffer(image))
	if err != nil {
		panic(err)
	}
	resp, err := client.Do(req)
	if err != nil {
		panic(err)
	}
	storeCount(api)
	println(resp)

	defer resp.Body.Close()
	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		panic(err)
	}

	res := string(body)
	println("res: ", res)
	if strings.Contains(res, "Max bytes limit exceeded") {
		res = "ImageTooLarge"
	}
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
