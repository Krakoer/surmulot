import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import "bootstrap/dist/css/bootstrap.min.css"
import "bootstrap"

/* import the fontawesome core */
import { library } from '@fortawesome/fontawesome-svg-core'

/* import font awesome icon component */
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'

/* import specific icons */
import { faPersonMilitaryRifle, faHouse } from '@fortawesome/free-solid-svg-icons'

/* add icons to the library */
library.add(faPersonMilitaryRifle, faHouse)

/* import specific icons */
createApp(App).use(router).component('font-awesome-icon', FontAwesomeIcon).mount('#app')
