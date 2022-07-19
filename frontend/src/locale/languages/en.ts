import { genMessage } from '../locales';
import elementLocale from 'element-plus/lib/locale/lang/en';

const modules = import.meta.globEager('./en/**/*.ts');
export default {
    ...genMessage(modules, 'en'),
    elementLocale,
};
