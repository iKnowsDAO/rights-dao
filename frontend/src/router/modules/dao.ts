import Home from '@/views/dao/home/Home.vue';
import DaoSubmit from '@/views/dao/DaoSubmit.vue';

const persons = [
    {
        path: '/dao',
        name: 'DaoHome',
        des: 'dao home',
        component: Home,
    },
    {
        path: '/dao/submit',
        name: 'DaoSubmit',
        des: 'dao submit',
        component: DaoSubmit,
    },
];

export default persons;
