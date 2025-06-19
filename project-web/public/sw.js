// sw.js
self.addEventListener('push', event => {
  const data = event.data?.json();
  const title = data?.title || 'Новое уведомление';
  const options = {
    body: data?.body || 'Вы получили новое сообщение!',
    // icon: '/icon.png',
    // badge: '/badge.png'
  };

  console.log("addEventListener push: ", event)
  console.log("addEventListener push data: ", data)

  self.registration.showNotification("test notification", {
    body: "Hey I am test!",
    // icon: "image.png",
  }).then(
    resp => {
      console.log("getSubscription: ", resp)
    },
    err => {
      console.log("getSubscription error: ", err)
    }
  )

  event.waitUntil(self.registration.showNotification(title, options));
});

// self.addEventListener('notificationclick', event => {
//   event.notification.close();
//   event.waitUntil(clients.openWindow('https://ваш-сайт'));
// });