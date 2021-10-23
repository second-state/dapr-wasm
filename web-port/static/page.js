var image;

function fileSelected(e) {
    const file = e.files[0];
    if (!file) {
        return;
    }

    if (!file.type.startsWith('image/')) {
        alert('Please select a image.');
        return;
    }

    const img = document.createElement('img-tag');
    img.file = file
    image = img;

    const reader = new FileReader();
    reader.onload = function(e) {
        var elem = document.getElementById("upload-pic");
        elem.src = e.target.result;
        elem.hidden = false;
        var origin_img = document.getElementById("origin-pic");
        origin_img.src = elem.src;
        var button = document.getElementById("run");
        button.removeAttribute("disabled");

    }
    reader.readAsDataURL(file);
}

function setButton() {
    var button = document.getElementById("run");
    if ("go" == getApi()) {
        button.innerText = "Classify";
    } else {
        button.innerText = "Grayscale";
    }
    button.disabled = false;
}

function setLoading(loading) {
    var button = document.getElementById("run");
    if (loading) {
        button.disabled = true;
        button.innerText = "Sending ...";
    } else {
        setButton();
    }
}

function setRes(res) {
    var elem = document.getElementById("result");
    elem.innerHTML = res;
    elem.hidden = false;
    var row = document.getElementById("grayscale-rows");
    row.hidden = true;
    var elem = document.getElementById("infer-rows");
    elem.hidden = false;
}

function setImageRes(data) {
    if (data == "ImageTooLarge") {
        alert("Image Too Large");
    } else {
        var row = document.getElementById("grayscale-rows");
        row.hidden = false;
        var elem = document.getElementById("infer-rows");
        elem.hidden = true;
        var img = document.getElementById("processed-pic");
        img.src = "data:image/png;base64, " + data;
        img.hidden = false;
        var origin_img = document.getElementById("origin-pic");
        origin_img.src = document.getElementById("upload-pic").src;
        origin_img.hidden = false;
    }
}

function getApi() {
    var select = document.getElementById('run-api');
    return select.options[select.selectedIndex].value;
}

function runWasm(e) {
    const reader = new FileReader();
    reader.onload = function(e) {
        setLoading(true);
        var req = new XMLHttpRequest();
        req.open("POST", '/api/hello', true);
        req.setRequestHeader('api', getApi());
        req.onload = function() {
            setLoading(false);
            if (req.status == 200) {
                var header = req.getResponseHeader("Content-Type");
                console.log(header);
                if (header == "image/png") {
                    setImageRes(req.response);
                } else {
                    setRes(req.response);
                }
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