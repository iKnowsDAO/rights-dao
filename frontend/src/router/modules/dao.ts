import Home from '@/views/dao/home/Home.vue';
import DaoSubmit from '@/views/dao/DaoSubmit.vue';
import Detail from '@/views/dao/detail/DaoDetail.vue';

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
    {
        path: '/dao/detail/:id',
        name: 'DaoDetail',
        des: 'dao detail',
        component: Detail,
    },
];

export default persons;
