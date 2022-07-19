import { Router } from 'vue-router';

export const goBack = (router: Router) => {
    window.history.length > 1 ? router.go(-1) : router.push('/');
};

export const goHome = (router: Router) => {
    router.push('/');
};

export const openTab = (url: string) => {
    window.open(url, '_blank');
};
