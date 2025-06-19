// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('./pkg');

let engine

const canvas = document.getElementById('wasm-example')

canvas.setAttribute('tabindex','0');
canvas.focus();
let newSW = null;

navigator.serviceWorker.register('/sw.js').then(reg => {
  reg.addEventListener('updatefound', () => {
    newSW = reg.installing;
    newSW.addEventListener('statechange', () => {
      if (newSW.state === 'installed') {
        // Показываем кнопку "Обновить"
        // showUpdateButton();
        // console.log('installed ok')
      }
    });
  });
});

const vapidKeys = {
  "publicKey":"BD7Vx5OCyye2u1mCgkN5dmotRj6F12NiviT9FXbH9ahIFYcsFy6zcYdsvN3WIMBTAncqptXm3ZUQwFfKAUxKAe0",
  "privateKey":"gei4NsFoTBLOZAFiaPdOLvJsZmUk6ufCU8w3JrBGSBk"
}


// main.js
if ('serviceWorker' in navigator) {
  navigator.serviceWorker.register('/sw.js')
    .then(registration => {
      // console.log('Service Worker зарегистрирован');

      // registration.pushManager.getSubscription().then(
      //   resp => {
      //     console.log("getSubscription: ", resp)
      //   },
      //   err => {
      //     console.log("getSubscription error: ", err)
      //   }
      // )

      const applicationServerKey = urlBase64ToUint8Array(vapidKeys.publicKey)
      return registration.pushManager.subscribe({
        userVisibleOnly: true,
        applicationServerKey
      });
    })
    .then(subscription => {
      // console.log('Подписка на push:', subscription);
      // Отправьте `subscription` на ваш сервер
      // Отправляем подписку на сервер (fetch-запрос)
      fetch('http://localhost:3000/subscribe', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(subscription)
      })
        .then(response => {
          if (!response.ok) throw new Error('Ошибка сохранения подписки');
          // console.log('Подписка сохранена на сервере!');
        })
        .catch(err => console.error('Ошибка:', err));
    })
    .catch(err => console.error('Ошибка:', err));
}

function urlBase64ToUint8Array(base64String) {
  // Заменяем URL-safe символы обратно на стандартные Base64
  const padding = '='.repeat((4 - (base64String.length % 4)) % 4);
  const base64 = (base64String + padding)
    .replace(/-/g, '+')
    .replace(/_/g, '/');

  // Декодируем Base64 строку в бинарные данные
  const rawData = atob(base64);

  // Создаем Uint8Array нужного размера
  const outputArray = new Uint8Array(rawData.length);

  // Заполняем массив значениями
  for (let i = 0; i < rawData.length; ++i) {
    outputArray[i] = rawData.charCodeAt(i);
  }

  return outputArray;
}

// При клике на кнопку обновляем SW
function updateSW() {
  if (newSW) {
    newSW.postMessage({ action: 'skipWaiting' });
  }
  window.location.reload(); // Перезагружаем страницу
}

// addEventListener("resize", (event) => { onresize(event) });

onresize = (event) => {
  if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
    canvas.height = window.innerHeight
    canvas.width = window.innerWidth
    if (engine) {
      engine.log("debug", "MsgFromJs", `canvas ${canvas.width} ${canvas.height}`)
    }
  }
}

rust
  .then(p => {
    // console.log(p)
    engine = p.get_engine()
    engine.run()
    // p.greet()
    // engine = p.init()
    // engine.run()
    // console.log(engine)
    // engine.start()
  })
  .catch(console.error)
  .catch((error) => {
    if (!error.message.startsWith("Using exceptions for control flow,")) {
      throw error;
    }
  })


// rust
//   .then(m => m.start())
//   .catch(console.error)
//   .catch((error) => {
//     if (!error.message.startsWith("Using exceptions for control flow,")) {
//       throw error;
//     }
//   })

// rust
//   .then(m => m.greet('World!'))
//   .catch(console.error);

// it not working, heh ----v
// import { greet } from './pkg'
// greet("Lol")