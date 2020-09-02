export function postData(url, data, callback) {
    let xhr = new XMLHttpRequest()
    xhr.onreadystatechange = function () {
        if (this.readyState !== 4) return
        callback(this.status, this.responseText)
    }
    xhr.open("POST", url, true)
    xhr.setRequestHeader("Content-Type", "application/json")
    xhr.send(JSON.stringify(data))
}

export function removeItemOnce(arr, value) {
    let index = arr.indexOf(value)
    if (index > -1) {
        arr.splice(index, 1)
    }
    return arr
}

export function getBasePath() {
    if (!process.env.NODE_ENV || process.env.NODE_ENV === "development") {
        return "http://localhost:8001/"
    } else {
        return "/"
    }
}
