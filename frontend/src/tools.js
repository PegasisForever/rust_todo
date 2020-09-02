export default function postData(url, data, callback) {
    let xhr = new XMLHttpRequest()
    xhr.onreadystatechange = function () {
        if (this.readyState !== 4) return
        callback(this.status, this.responseText)
    }
    xhr.open("POST", url, true)
    xhr.setRequestHeader("Content-Type", "application/json")
    xhr.send(JSON.stringify(data))
}
