
var image;

function fileSelected(e) {
    const file = e.files[0];
    if (!file) {
        return;
    }

    if (!file.type.startsWith('image/jpeg')) {
        alert('Please select a jpeg image.');
        return;
    }

    const img = document.createElement('img-tag');
    img.file = file
    image = img;

    const reader = new FileReader();
    reader.onload = function (e) {
        var elem = document.getElementById("upload-pic");
        elem.src = e.target.result;
        elem.hidden = false;
        var button = document.getElementById("run");
        button.removeAttribute("disabled");

    }
    reader.readAsDataURL(file);
}

function setLoading(loading) {
    var button = document.getElementById("run");
    if (loading) {
        button.disabled = true;
        button.innerText = "Sending ...";
    } else {
        button.disabled = false;
        button.innerText = "Classify with WASM";
    }
}

function setRes(res) {
    var elem = document.getElementById("result");
    elem.innerHTML = res;
    elem.hidden = false;
}

function getApi() {
    var select = document.getElementById('run-api');
    return select.options[select.selectedIndex].value;
}

function runWasm(e) {
    const reader = new FileReader();
    reader.onload = function (e) {
        setLoading(true);
        var req = new XMLHttpRequest();
        req.open("POST", '/api/hello', true);
        req.setRequestHeader('api', getApi());
        req.onload = function () {
            setLoading(false);
            if (req.status == 200) {
                setRes(req.response);
            } else {
                setRes("API error with status: " + req.status);
            }
        };
        const blob = new Blob([e.target.result], {
            type: 'application/octet-stream'
        });
        req.send(blob);
    };
    console.log(image.file)
    reader.readAsArrayBuffer(image.file);
}
