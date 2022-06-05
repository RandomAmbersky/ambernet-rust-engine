export function sayHello () {
	console.log('Hello from helpers')
	// return Promise.resolve()
}

export async function loadImage(src) {
	return new Promise((resolve, reject) => {
		const img = new Image();
		img.addEventListener("load", () => resolve(img));
		img.addEventListener("error", err => reject(err));
		img.src = src;
	});
}
