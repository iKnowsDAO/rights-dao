import {ElMessage} from "element-plus/es";

export const showMessageError = (message: string) =>
    ElMessage({
        showClose: true,
        message,
        type: 'error',
    });
export const showMessageSuccess = (message: string) =>
    ElMessage({
        showClose: true,
        message,
        type: 'success',
    });
