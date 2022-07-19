import Submit from '@/views/post/Submit.vue';
import Detail from '@/views/post/Detail.vue';

const post = [
    {
        path: '/post/submit',
        name: 'Submit',
        des: 'post submit',
        component: Submit,
    },
    {
        path: '/post/detail/:id',
        name: 'Detail',
        des: 'post detail',
        component: Detail,
    },
];

export default post;
