async function loadBinary (uri) {
	const binary = await load(uri, 'arraybuffer')
	return new Uint8Array(binary)
}

async function load (uri, responseType = "text") {
	return new Promise( function (resolve, reject){
		const xhr = new XMLHttpRequest()
		xhr.open("GET", uri, true)
		xhr.responseType = responseType;
		xhr.onload = function () {
			if (this.status >= 200 && this.status < 300) {
				resolve(xhr.response)
			} else {
				reject({
					status: xhr.status,
					statusText: xhr.statusText
				})
			}
		}
		xhr.onerror = function () {
			reject({
				status: xhr.status,
				statusText: xhr.statusText
			})
		}
		xhr.send()
	})
}

module.exports = {
	loadBinary
}
