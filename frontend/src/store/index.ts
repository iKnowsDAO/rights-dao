import { createStore } from 'vuex';

import user from './modules/user';

export const UserText = 'user';

export default createStore({
    modules: {
        user,
    },
});
