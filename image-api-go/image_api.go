package main

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"os"

	"github.com/second-state/WasmEdge-go/wasmedge"
	bindgen "github.com/second-state/wasmedge-bindgen/host/go"

	"github.com/dapr/go-sdk/service/common"
	daprd "github.com/dapr/go-sdk/service/http"
)

// initVM initialize WasmEdge's VM
func initVM() (*wasmedge.Configure, *wasmedge.VM) {
	wasmedge.SetLogErrorLevel()
	/// Set Tensorflow not to print debug info
	os.Setenv("TF_CPP_MIN_LOG_LEVEL", "3")
	os.Setenv("TF_CPP_MIN_VLOG_LEVEL", "3")

	/// Create configure
	vmConf := wasmedge.NewConfigure(wasmedge.WASI)

	/// Create VM with configure
	vm := wasmedge.NewVMWithConfig(vmConf)

	/// Init WASI
	var wasi = vm.GetImportModule(wasmedge.WASI)
	wasi.InitWasi(
		os.Args[1:],     /// The args
		os.Environ(),    /// The envs
		[]string{".:."}, /// The mapping directories
	)

	/// Register WasmEdge-tensorflow and WasmEdge-image
	var tfobj = wasmedge.NewTensorflowImportObject()
	var tfliteobj = wasmedge.NewTensorflowLiteImportObject()
	vm.RegisterModule(tfobj)
	vm.RegisterModule(tfliteobj)
	var imgobj = wasmedge.NewImageImportObject()
	vm.RegisterModule(imgobj)

	/// Instantiate wasm
	vm.LoadWasmFile("./lib/classify.so")
	vm.Validate()

	return vmConf, vm
}

// currently don't use it, only for demo
func imageHandler(_ context.Context, in *common.InvocationEvent) (out *common.Content, err error) {
	image := string(in.Data)
	vmConf, vm := initVM()
	bg := bindgen.Instantiate(vm)
	defer bg.Release()
	defer vm.Release()
	defer vmConf.Release()

	// recognize the image
	res, err := bg.Execute("infer", image)
	fmt.Printf("Image classify result: %q\n", res)
	out = &common.Content{
		Data:        []byte(string(res[0].([]byte))),
		ContentType: in.ContentType,
		DataTypeURL: in.DataTypeURL,
	}
	return out, nil
}

func main() {
	s := daprd.NewService(":9003")

	if err := s.AddServiceInvocationHandler("/api/image", imageHandler); err != nil {
		log.Fatalf("error adding invocation handler: %v", err)
	}

	if err := s.Start(); err != nil && err != http.ErrServerClosed {
		log.Fatalf("error listenning: %v", err)
	}
}
