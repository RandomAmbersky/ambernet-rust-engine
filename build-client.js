const Bundler = require('parcel-bundler')
const Path = require('path')
const app = require('express')()

const port = process.env.PORT || 9000;

// via https://ru.parceljs.org/api.html
// Путь к файлу точки входа
const file = Path.join(__dirname, './web/index.html')

// Опции упаковщика (Bundler)
const options = {
  // outDir: './dist', // Каталог для файлов сборки, по умолчанию - dist
  // outFile: 'index.html', // Имя выходного файла
  // publicUrl: './', // Путь, который обслуживает сервер, по умолчанию - '/'
  // watch: true, // следует ли отслеживать изменения файлов и пересобирать их при изменении, по умолчанию - process.env.NODE_ENV !== 'production'
  // cache: true, // Включает или отключает кеширование, по умолчанию - true
  // cacheDir: '.cache', // Каталог кеширования, по умолчанию .cache
  // minify: false, // Минизировать файлы, включено если process.env.NODE_ENV === 'production'
  // target: 'browser', // browser/node/electron, по умолчанию - browser
  // https: false, // Использовать защищённое соединение (https) для файлов, по умолчанию - false
  logLevel: 3, // 3 = логировать всё, 2 = логировать предупреждения и ошибки, 1 = логировать ошибки
  // hmrPort: 0, // Порт на котором работает сокет hmr, по умолчанию - случайный свободный порт (0 в node.js определяет случайный свободный порт)
  // sourceMaps: false, // Включить или отключить sourcemaps, по умолчанию включено (пока ещё не поддерживается в минифицированных сборках)
  // hmrHostname: '', // Имя хоста для модуля горячей перезагрузки, по умолчанию - ''
  // detailedReport: true // Распечатывает подробный отчёт о бандлах, ресурсах, размерах файлов и времени, по умолчанию - false, отчёты распечатываются, если watch отключен
}

// Инициализация нового упаковщика с использованием файла и опций
const bundler = new Bundler(file, options);

// Путь express использует мидлвар упаковщика, это позволит Parcel обрабатывать каждый запрос к вашему серверу express
app.use(bundler.middleware())

// Сервер прослушивает порт 8080
app.listen(port, () => {
  console.log(`App now listening on port ${port}`);
})
