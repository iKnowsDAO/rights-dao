<template>
    <div class="dao-submit-container">
        <Navigator/>
        <div class="container">
            <div class="submit-title">
                <h3 class="title">
                    {{ $t('dao.proposal') }}
                    <div class="back-button" @click="$router.back()"
                    >{{ '<' }}{{ $t('common.back') }}
                    </div>
                </h3>
            </div>
            <el-row class="post-form">
                <el-col :sm={span:16,offset:4} :xs="24">
                    <el-form :model="form" hide-required-asterisk
                             ref="ruleFormRef">
                        <el-form-item prop="title" :label="$t('dao.form.title.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('dao.form.title.placeholder'),
                                        trigger: 'blur'}]"
                        >
                            <el-input v-model="form.title"
                                      maxlength="200"
                                      show-word-limit
                                      :placeholder="$t('dao.form.title.placeholder')"/>
                        </el-form-item>
                        <el-form-item :label="$t('dao.form.content.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('dao.form.content.placeholder'),
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
                        <el-form-item :label="$t('dao.form.action.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('dao.form.action.placeholder'),
                                        trigger: 'blur'}]"
                                      prop="action">
                            <el-select class="i-select"
                                       popper-class="i-select-pop"
                                       v-model="form.action"
                                       :placeholder="$t('dao.form.action.placeholder')"
                            >
                                <el-option
                                    v-for="item in action"
                                    :key="item.value"
                                    :label="item.label"
                                    :value="item.value"
                                />
                            </el-select>
                        </el-form-item>
                        <el-form-item v-if="form.action"
                                      :label="$t('dao.form.related.label')"
                                      :rules="[{
                                        required: true,
                                        message: $t('dao.form.related.placeholder'),
                                        trigger: 'blur'}]"
                                      prop="related">
                            <el-input v-model="form.related"
                                      maxlength="63"
                                      show-word-limit
                                      :placeholder="$t('dao.form.related.placeholder')">
                            </el-input>
                            <span>{{ $t('dao.form.action.tips') }}</span>
                        </el-form-item>
                        <el-form-item :label="$t('post.help.endTime.label')">
                            <el-config-provider :locale="elementPlusLocale">
                                <el-date-picker
                                    v-model="form.deadline"
                                    type="datetime"
                                    :placeholder="$t('post.help.endTime.placeholder')"
                                    popper-class="i-date-pop"
                                    :editable="false"
                                    value-format="x"
                                    :disabledDate="disabledDateFun"
                                />
                            </el-config-provider>
                            <br/>
                            <span>{{ $t('dao.form.deadline.tips') }}</span>
                        </el-form-item>
                    </el-form>
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
    import {ref, onMounted, computed, nextTick} from 'vue';
    import Navigator from '@/components/navigator/Navigator.vue';
    import {
        ElRow, ElCol, ElButton, ElSelect, ElOption, ElForm, ElFormItem, ElInput, ElMessage, ElConfigProvider,
        ElDatePicker, ElLoading
    } from 'element-plus/es';
    import {Close} from '@element-plus/icons-vue';
    import type {FormInstance, FormRules} from 'element-plus'
    import {SupportedLocale, t} from '@/locale';
    import {Quill, QuillEditor} from '@vueup/vue-quill';
    import ImageUploader from "quill-image-uploader";
    import {useRouter} from 'vue-router';
    import en from 'element-plus/lib/locale/lang/en';
    import zhCn from 'element-plus/lib/locale/lang/zh-cn';
    import {goBack} from "@/router/routers";
    import {showMessageError, showMessageSuccess, showResultError} from "@/utils/message";
    import {calculatedICPIdLength, uploadImage} from "@/utils/images";
    import {addDaoProposal} from "@/api/dao";
    import { useUserStore } from "@/stores/user";

    const userStore = useUserStore();
    const router = useRouter();

    const locale = computed(() => userStore.getLocale);
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
        action: "",
        related: "",// 提案目标的principalId，每次提案只能有一个人
        deadline: "",
    });
    const action = ref([{
        value: "Add",
        label: t('dao.form.action.addAdmin')
    }])
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
        placeholder: '......',       //placeholder,在双语切换时不会即时响应，注释了
        readyOnly: false, //是否只读
        theme: 'snow', //主题 snow/bubble
        syntax: true, //语法检测
    };
    const elementPlusLocale = computed(() => {
        switch (locale.value) {
            case SupportedLocale.zhCN:
                return zhCn;
            default:
                return en;
        }
    });

    onMounted(() => {
        init()
    });

    const showEditorLength = computed(() => {
        const length = calculatedICPIdLength(form.value.content.content);
        length > limitLength ? (isEditorErr.value = true) : (isEditorErr.value = false);
        return length;
    });

    const disabledDateFun = (time) => {
        let oneDay = 24 * 3600 * 1000;
        //管理员提案时间应只能最短2天，最长7天
        return !(time.getTime() < Date.now() + 7 * oneDay && time.getTime() > Date.now() + 2 * oneDay);
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
                let dao = {
                    id: form.value.related,
                    ...form.value
                };
                // delete dao.related;
                if (dao.deadline) {
                    //结束时间，传到后端需要扩大一下位数
                    //@ts-ignore
                    dao.deadline = Number(dao.deadline)* 1000 * 1000;
                }
                console.log("dao", dao);
                addDaoProposal(dao).then(res => {
                    console.log("dao result:", res);
                    if (res.Ok) {
                        showMessageSuccess(t('message.dao.create'));
                        router.push('/dao/detail/' + Number(res.Ok));
                    } else {
                        showResultError(res);
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
    }

</script>

<style lang="scss">
    .dao-submit-container {
        /* 当页面宽度小于426px*/
        @media screen and (max-width:426px) {
            .container{
                padding: 0 10px;
            }
            .post-form .el-form-item{
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
            .el-form-item__content{
                display: unset!important;
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
