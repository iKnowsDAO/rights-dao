<template>
    <div class="post-detail-write-reply-container">
        <div class="container">
            <el-row>
                <el-col :sm={span:16,offset:4} :xs="24">
                    <el-card :class="{ isEditorError: isEditorErr }">
                        <h4 style="margin-bottom: 5px">{{t('post.answer')}}</h4>
                        <QuillEditor
                            ref="myTextEditor"
                            v-model:content="reply"
                            contentType="html"
                            :options="editorOption"
                            @update:content="isEditorChange = true"
                        />
                        <div v-if="isEditorErr" class="errorTip">
                            {{t('message.post.error')}}
                        </div>
                        <div class="footer">
                            <div>
                                <span class="fold" @click="foldReply">{{t('post.answerFold')}}</span>
                            </div>
                            <div>
                                <span class="editorCalculate">
                                    {{ showEditorLength }} / {{ limitLength }}
                                </span>
                                <el-button type="primary" @click="submit()" :loading="loading">{{t('post.answerSubmit')}}</el-button>
                            </div>
                        </div>
                    </el-card>
                </el-col>
            </el-row>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {ref, computed, defineEmits} from 'vue';
    import {ElRow, ElCol, ElButton, ElCard} from 'element-plus/es';
    import Avatar from '@/components/common/Avatar.vue';
    import {Quill, QuillEditor} from '@vueup/vue-quill';
    import ImageUploader from "quill-image-uploader";
    import {t} from '@/locale';
    import {calculatedICPIdLength, uploadImage} from "@/utils/images";
    import {addPostReply} from "@/api/post";
    import {useRoute} from "vue-router";
    import {showMessageSuccess} from "@/utils/message";

    const editorOption = {
        modules: {
            toolbar: {
                container: [
                    ['bold', 'italic', 'underline', 'strike'], // 加粗 斜体 下划线 删除线
                    ['blockquote', 'code-block'], // 引用  代码块
                    [{header: 1}, {header: 2}], // 1、2 级标题
                    [{list: 'ordered'}, {list: 'bullet'}], // 有序、无序列表
                    [{script: 'sub'}, {script: 'super'}], // 上标/下标
                    // [{ indent: "-1" }, { indent: "+1" }], // 缩进
                    // [{'direction': 'rtl'}],                         // 文本方向
                    // [{ size: ["small", false, "large", "huge"] }], // 字体大小
                    [{header: [1, 2, 3, 4, 5, 6, false]}], // 标题
                    // [{ color: [] }, { background: [] }], // 字体颜色、字体背景颜色
                    // [{ font: [] }], // 字体种类
                    // [{ align: [] }], // 对齐方式
                    ['clean'], // 清除文本格式
                    ['image'], // 图片
                    // ["link", "image","video"], // 链接、图片、视频
                ], //工具菜单栏配置
            },
            imageUploader: {
                upload: (file) => {
                    return new Promise((resolve, reject) => {
                        uploadImage(file).then(res => {
                                if (res!=='') {
                                    resolve(res)
                                } else {
                                    reject()
                                }
                            }
                        )
                    });
                }
            },
        },
        placeholder: '......',       //placeholder,在双语切换时不会即时响应
        readyOnly: false, //是否只读
        theme: 'snow', //主题 snow/bubble
        syntax: true, //语法检测
    };
    const loading = ref(false);
    const route = useRoute();
    const postId = Number(route.params.id);
    //编辑器是否发生变化
    const isEditorChange = ref(false);
    const isEditorErr = ref(false);
    Quill.register("modules/imageUploader", ImageUploader);
    //限制输入长度10000个字
    const limitLength = 10000;
    // 直接取出，没有额外逻辑，用 computed 变成响应式值
    const myTextEditor = ref<{ setHTML: Function; getText: Function } | null>(null);
    const reply = ref('');

    const emit = defineEmits(['foldWrite', 'replySuccess'])
    const foldReply = () => {
        emit('foldWrite');
    }

    const showEditorLength = computed(() => {
        // 这个返回的字数是专门把图片上传到后端，用特殊字符串取代，放以后再看看
        // 放在第一位，免得computed不知道计算那个属性改变
        const length = calculatedICPIdLength(reply.value);
        // 没有完成初始化时，直接使用myTextEditor里的方法会报错。
        // 如果内容为空，就返回0
        if(myTextEditor.value && myTextEditor.value.getText().trim().length===0){
            return 0;
        }
        length > limitLength ? (isEditorErr.value = true) : (isEditorErr.value = false);
        return length;
    });

    const submit = () => {
        if (reply.value.length === 0) {
            //内容为空不准提交
            isEditorErr.value = true;
            return
        }
        loading.value = true;
        addPostReply(postId, reply.value).then(res => {
            console.log(res);
            if (res.Ok) {
                emit('replySuccess');
                showMessageSuccess(t('message.post.reply'));
                emit('foldWrite');
                //将内容清空
                myTextEditor.value?.setHTML('');
            }
        }).finally(() => {
            loading.value = false;
        })
    }

</script>
<style lang="scss">
    .post-detail-write-reply-container {
        .isEditorError{
            .ql-toolbar{
                border: 1px solid var(--el-color-danger);
            }
            .ql-container {
                border: 1px solid var(--el-color-danger);
            }
            .errorTip,.editorCalculate{
                color: var(--el-color-danger)!important;
            }
        }
        .el-card {
            position: relative;
            margin-top: 10px;
            .footer {
                display: flex;
                justify-content: space-between;
                align-items: center;
                margin-top: 10px;
                .fold {
                    color: var(--el-color-info);
                    &:hover {
                        cursor: pointer;
                    }
                }
                .editorCalculate {
                    margin-right: 10px;
                    color: var(--el-color-info);
                    /*font-size: 12px;*/
                    /*position: absolute;*/
                    /*right: 25px;*/
                    /*bottom: 25px;*/
                }
            }
        }
    }
</style>
