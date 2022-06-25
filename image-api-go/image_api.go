package main

import (
	"bytes"
	"context"
	"fmt"
	"log"
	"net/http"
	"os/exec"
	"strings"

	"github.com/second-state/WasmEdge-go/wasmedge"
	bindgen "github.com/second-state/wasmedge-bindgen/host/go"

	"github.com/dapr/go-sdk/service/common"
	daprd "github.com/dapr/go-sdk/service/http"
)

func imageHandlerWASI(_ context.Context, in *common.InvocationEvent) (out *common.Content, err error) {
	/// Set not to print debug info
	wasmedge.SetLogErrorLevel()

	/// Create configure
	var conf = wasmedge.NewConfigure(wasmedge.REFERENCE_TYPES)
	conf.AddConfig(wasmedge.WASI)

	/// Create VM with configure
	var vm = wasmedge.NewVMWithConfig(conf)

	vm.LoadWasmFile("./lib/grayscale_lib_origin.wasm")
	vm.Validate()
	/// vm.Instantiate()
	bg := bindgen.Instantiate(vm)

	image := ""
	for i := 0; i < len(in.Data); i++ {
		image += fmt.Sprintf("%d,", in.Data[i])
	}
	sz := len(image)
	if sz > 0 && image[sz-1] == ',' {
		image = image[:sz-1]
	}

	res, err := bg.Execute("grayscale", image)
	// ans := string(res[0].([]byte))
	ans := res[0].(string)
	if err != nil {
		println("error: ", err.Error())
	}

	bg.Release()
	vm.Release()
	conf.Release()

	fmt.Printf("Image result: %q\n", len(ans))
	out = &common.Content{
		Data:        []byte(ans),
		ContentType: in.ContentType,
		DataTypeURL: in.DataTypeURL,
	}
	return out, nil
}

// currently don't use it, only for demo
func imageHandler(_ context.Context, in *common.InvocationEvent) (out *common.Content, err error) {
	image := string(in.Data)
	cmd := exec.Command("./lib/wasmedge-tensorflow-lite", "./lib/classify.so")
	cmd.Stdin = strings.NewReader(image)

	var std_out bytes.Buffer
	cmd.Stdout = &std_out
	cmd.Run()
	if err != nil {
		log.Fatal(err)
	}

	res := std_out.String()
	fmt.Printf("Image classify result: %q\n", res)
	out = &common.Content{
		Data:        []byte(res),
		ContentType: in.ContentType,
		DataTypeURL: in.DataTypeURL,
	}
	return out, nil
}

func main() {
	s := daprd.NewService(":9003")

	if err := s.AddServiceInvocationHandler("/api/image", imageHandlerWASI); err != nil {
		log.Fatalf("error adding invocation handler: %v", err)
	}

	if err := s.Start(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("error listenning: %v", err)
	}
}
