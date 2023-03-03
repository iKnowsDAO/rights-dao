// import Error from '../../views/errors/Error.vue';
import Error404 from '@/views/errors/Error404.vue';

const errors = [
    // {
    //     path: '/error',
    //     name: 'Error',
    //     des: 'error',
    //     component: Error,
    // },
    {
        path: '/error/404',
        name: 'Error404',
        des: 'error 404',
        component: Error404,
    },
];

export default errors;
