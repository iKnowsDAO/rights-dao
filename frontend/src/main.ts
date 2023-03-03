import 'element-plus/dist/index.css'; // element plus 生效的样式
import '@vueup/vue-quill/dist/vue-quill.snow.css'; //导入quill编辑器样式
// import '@purge-icons/generated'; // 导入 icons
import { createApp } from 'vue';
import i18n from './locale';
import router from './router';
import App from './App.vue';
import { ElInfiniteScroll } from "element-plus/es";

import { VueClipboard } from '@soerenmartius/vue3-clipboard'; //复制到粘贴板
import { createPinia } from "pinia"; //类似于vuex的状态管理

const app = createApp(App);
const pinia = createPinia();

app.use(i18n).use(router).use(VueClipboard).use(ElInfiniteScroll).use(pinia);
app.mount('#app');

