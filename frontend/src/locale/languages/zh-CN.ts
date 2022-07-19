import { genMessage } from '../locales';
import elementLocale from 'element-plus/lib/locale/lang/zh-cn';

const modules = import.meta.globEager('./zh-CN/**/*.ts');
export default {
    ...genMessage(modules, 'zh-CN'),
    elementLocale,
};
