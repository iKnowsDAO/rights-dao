<template>
    <div class="right-menu-container">
        <el-card shadow="never">
            <el-button type="primary" class="create-button" @click="onClick()">
                <span v-if="buttonType==='post'">{{t('post.help.create')}}</span>
                <span v-else-if="buttonType==='dao'">{{t('dao.create')}}</span>
            </el-button>
            <div class="beta">
                <b class="flex-y-center">
                    <el-icon>
                        <Opportunity/>
                    </el-icon>
                    Beta Warning!</b>
                <span class="text">RightsDao is in the Beta phase, there may be issues.</span>
            </div>
            <el-divider/>
            <div class="public-item">
                <a href="https://dfinity.org/">
                    More about the Internet Computer
                </a>
            </div>
        </el-card>
        <Footer/>
    </div>
</template>

<script lang="ts" setup>
import { PropType, defineProps, ref, computed, watch, onMounted } from 'vue';
import { ElButton, ElCard, ElIcon, ElDivider } from 'element-plus/es';
import { Opportunity } from '@element-plus/icons-vue';
import Footer from '@/components/footer/Footer.vue';
import {t} from '@/locale';
import { ElAvatar } from 'element-plus/es';
import { useRouter } from 'vue-router';
import { openTab } from '@/router/routers';
import router from "@/router";

const props = defineProps({
    buttonType: {
        type: String,
        required: true,
    },
});

const onClick = () => {
    if (props.buttonType === 'post') {
        router.push('/post/submit')
    } else if (props.buttonType === 'dao') {
        router.push('/dao/submit')
    }
};

</script>

<style lang="scss">
.right-menu-container {
    .beta {
        margin-top: 24px;
        padding: 8px 16px;
        color: #e6a23c;
        background-color: #faecd8;
        border-color: #faecd8;
        border-radius: 10px;
        .text {
            font-size: 14px;
        }
    }
    .public-item {
        font-size: 14px;
        a {
            color: black;
            text-decoration: none;
            &:hover {
                opacity: 0.8;
                text-decoration: underline;
            }
        }
    }
    .create-button {
        width: 100%;
        min-height: 40px;
    }
}
</style>
