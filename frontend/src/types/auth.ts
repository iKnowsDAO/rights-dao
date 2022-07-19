import { ref } from 'vue';

export class Auth {
    client: any = null; // 与 II 的链接对象 这个连接对象貌似不能被 vue 代理会出错
    clientReady = ref<boolean>(false); // 标识链接对象是否完成
    signedIn = ref<boolean>(false); // 是否已经登录
}
