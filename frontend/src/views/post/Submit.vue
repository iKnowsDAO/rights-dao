<template>
    <div class="post-submit-container">
        <Navigator/>
        <div class="container">
            <div class="submit-title">
                <h3 class="title">
                    {{ $t('post.help.create') }}
                    <div class="back-button" @click="$router.back()"
                    >{{ '<' }}{{ $t('common.back') }}
                    </div>
                </h3>
            </div>
            <el-row class="post-form">
                <el-col :sm={span:16,offset:4} :xs="24">
                    <el-form :model="form" hide-required-asterisk
                             ref="ruleFormRef">
                        <el-form-item prop="title" :label="$t('post.help.title.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('post.help.title.placeholder'),
                                        trigger: 'blur'}]"
                        >
                            <el-input v-model="form.title"
                                      maxlength="200"
                                      show-word-limit
                                      :placeholder="$t('post.help.title.placeholder')"/>
                        </el-form-item>
                        <el-form-item :label="$t('post.help.content.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('post.help.content.placeholder'),
                                        trigger: 'blur'}]"
                                      :class="{ isEditorError: isEditorErr }"
                                      prop="content.content">
                            <QuillEditor
                                ref="myTextEditor"
                                v-model:content="form.content.content"
                                contentType="html"
                                :options="editorOption"
                                @update:content="isEditorChange = true"
                            />
                            <span class="editorCalculate" :class="{ isCalcError: isEditorErr }">
                            {{ showEditorLength }} / {{ limitLength }}
                        </span>
                        </el-form-item>
                        <el-form-item :label="$t('post.help.category.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('post.help.category.placeholder'),
                                        trigger: 'blur'}]"
                                      prop="category">
                            <el-select class="i-select"
                                       popper-class="i-select-pop"
                                       v-model="form.category"
                                       :placeholder="$t('post.help.category.placeholder')"
                            >
                                <el-option
                                    v-for="item in category"
                                    :key="item.value"
                                    :label="item.label"
                                    :value="item.value"
                                />
                            </el-select>
                        </el-form-item>
                        <el-form-item v-for="(item, index) in form.participants"
                                      :key="index"
                                      :prop="'participants.' + index"
                                      :rules="[{
                                        required: true,
                                        message: $t('post.help.participants.placeholder'),
                                        trigger: 'blur'}]"
                                      prop="participants[0]">
                            <template #label>
                                <span v-if="index===0">{{$t('post.help.participants.label')}}</span>
                                <span v-else></span>
                            </template>
                            <el-input v-model="form.participants[index]"
                                      maxlength="30"
                                      show-word-limit
                                      :placeholder="$t('post.help.participants.placeholder')">
                                <template #append>
                                    <el-button :icon="Close" @click.prevent="removeItem(index)"></el-button>
                                </template>
                            </el-input>
                        </el-form-item>
                    </el-form>
                    <div style="display: flex;justify-content: space-between">
                        <el-button @click="addParticipants">{{t('post.help.participants.add')}}</el-button>
                    </div>
                </el-col>
            </el-row>
            <div style="text-align: center" class="form-footer">
                <el-button type="primary" class="submit-button" @click="submit(ruleFormRef)" :loading="loading">
                    {{t('post.submit')}}
                </el-button>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
    import { ref, onMounted, watch, computed, nextTick } from 'vue';
    import Navigator from '@/components/navigator/Navigator.vue';
    import {
        ElRow, ElCol, ElButton, ElSelect, ElOption, ElForm, ElFormItem, ElInput, ElMessage, ElConfigProvider,
        ElDatePicker, ElLoading, ElRadio, ElRadioGroup
    } from 'element-plus/es';
    import { Close } from '@element-plus/icons-vue';
    import type { FormInstance, FormRules } from 'element-plus'
    import { t } from '@/locale';
    import { Quill, QuillEditor } from '@vueup/vue-quill';
    import ImageUploader from "quill-image-uploader";
    import { useRouter } from 'vue-router';
    import { submitPost } from "@/api/post";
    import { goBack } from "@/router/routers";
    import { showMessageError, showMessageSuccess } from "@/utils/message";
    import { calculatedICPIdLength, uploadImage } from "@/utils/images";
    import { useUserStore } from "@/stores/user";

    const userStore = useUserStore();
    const router = useRouter();

    const currentUserPrincipal = computed<string>(() => userStore.principal);
    const loading = ref(false);
    //编辑器是否发生变化
    const isEditorChange = ref(false);
    const isEditorErr = ref(false);
    //注册quill上传图片模块
    Quill.register("modules/imageUploader", ImageUploader);
    //限制输入长度10000个字
    const limitLength = 10000;
    const ruleFormRef = ref<FormInstance>();
    // 直接取出，没有额外逻辑，用 computed 变成响应式值
    const myTextEditor = ref<{ setHTML: Function; getText: Function } | null>(null);
    const form = ref({
        title: "",
        content: {
            content: "",
            format: "html"
        },
        photos: [],
        category: "",
        participants: [] as string[],//期待参与者
        end_time: [] as number[],
    });
    const category = ref([{
        value: "Tech",
        label: t('post.help.category.tech')
    }, {
        value: "Law",
        label: t('post.help.category.law')
    }, {
        value: "Safeguard",
        label: t('post.help.category.safeguard')
    }, {
        value: "Blacklist",
        label: t('post.help.category.blacklist')
    }, {
        value: "Other",
        label: t('post.help.category.other')
    }]);
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
                                if (res !== '') {
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
        placeholder: '......',       //placeholder,在双语切换时不会即时响应，注释了
        readyOnly: false, //是否只读
        theme: 'snow', //主题 snow/bubble
        syntax: true, //语法检测
    };

    onMounted(() => {
        init()
    });

    const saveDraftBox = () => {
        //存草稿箱
        if (showEditorLength.value > 0) {
            localStorage.setItem('postDraftBox', JSON.stringify({form: form.value, time: new Date().toLocaleString()}));
        }
    }

    const getDraftBox = () => {
        //读取并设置草稿箱中的内容
        const item = localStorage.getItem('postDraftBox');
        if (!item) {
            return
        }
        const postDraftBox = JSON.parse(item);
        if (postDraftBox && myTextEditor.value) {
            form.value = postDraftBox.form;
            showMessageSuccess(t('post.draft.get') + postDraftBox.time)
            myTextEditor.value.setHTML(form.value.content.content);
        }
    }

    const showEditorLength = computed(() => {
        const length = calculatedICPIdLength(form.value.content.content);
        length > limitLength ? (isEditorErr.value = true) : (isEditorErr.value = false);
        return length;
    });

    const addParticipants = () => {
        form.value.participants.push("");
    }

    const removeItem = (index) => {
        if (index !== -1) {
            form.value.participants.splice(index, 1)
        }
    }

    const submit = async (formEl: FormInstance | undefined) => {
        console.log("submit formEl", formEl)
        if (!formEl) return;
        await formEl.validate((valid, fields) => {
            if (valid && !isEditorErr.value) {
                const fullLoading = ElLoading.service({
                    target: ".container",
                    lock: true
                });
                loading.value = true;
                console.log("form", form.value);
                let post = {...form.value};
                if (post.end_time[0]) {
                    //结束时间，暂时没用了
                    post.end_time[0] *= 1000 * 1000;
                }
                submitPost(post).then(res => {
                    console.log(res);
                    if (res.Ok) {
                        showMessageSuccess(t('message.post.create'));
                        router.push('/post/detail/' + Number(res.Ok));
                        //发布成功后删除草稿箱里的内容。
                        localStorage.removeItem('postDraftBox')
                    }
                }).finally(() => {
                    loading.value = false;
                    fullLoading.close();
                })
            } else {
                console.error('error submit!', fields)
            }
        })
    }

    watch(
        () => form,
        () => {
            //监听form对象，有变动就存草稿箱。
            saveDraftBox();
        },
        {deep: true}
    );

    const init = () => {
        console.log("currentUserPrincipal.value", currentUserPrincipal.value)
        //验证是否登录
        setTimeout(() => {
            nextTick(() => {
                if (!currentUserPrincipal.value) {
                    showMessageError(t('message.error.noLogin'));
                    // setTimeout(() => {
                    //等用户看清了错误提示再弹
                    goBack(router);
                    // }, 1500);
                }
            });
        }, 1500);
        getDraftBox();
    }

</script>

<style lang="scss">
    .post-submit-container {
        /* 当页面宽度小于426px*/
        @media screen and (max-width: 426px) {
            .container {
                padding: 0 10px;
            }
            .post-form .el-form-item {
                display: block;
            }
        }
        .container {
            .is-error, .isEditorError {
                .ql-toolbar {
                    border: 1px solid var(--el-color-danger);
                    border-bottom: 0;
                }
                .ql-container {
                    border: 1px solid var(--el-color-danger);
                }
            }
            .submit-title {
                margin-top: 20px;
                text-align: center;
                .title {
                    position: relative;
                }
                .back-button {
                    position: absolute;
                    right: 0;
                    top: 0;
                    font-size: 1.75rem;
                    &:hover {
                        cursor: pointer;
                    }
                }
            }
            .post-form {
                margin-top: 20px;
                .editorCalculate {
                    color: var(--el-color-info);
                    font-size: 12px;
                    position: absolute;
                    right: 13px;
                    bottom: 0px;
                }
            }
            .form-footer {
                margin-top: 10px;
                .submit-button {
                    font-size: 1.45rem;
                    min-width: 110px;
                    min-height: 35px;
                }
            }
        }
    }

</style>
