<template>
    <div class="person-profile-container">
        <Navigator ref="childRef"/>
        <ProfileUser @username="getName" @editProfile="refreshUser"/>
        <ProfileList :username="username"/>
    </div>
</template>
<script lang="ts" setup>
    import {ref} from 'vue';
    import Navigator from '@/components/navigator/Navigator.vue';
    import ProfileUser from './modules/ProfileUser.vue';
    import ProfileList from './modules/ProfileList.vue';

    const username = ref('');
    // 获取子组件
    const childRef = ref(null);

    const getName = (value) => {
        username.value = value;
    }

    const refreshUser = () => {
        //@ts-ignore
        childRef.value.getUserInfoFromServices();// 调用子组件的方法
    }

</script>
<style lang="scss">
    .person-profile-container{
        .user-edit-dialog{
            .el-form{
                .el-form-item{
                    .el-form-item__label{
                        padding: 6px 12px 0 0;
                    }
                }
                .el-form-item__content{
                    .el-input-group__append{
                        .el-button{
                            display: flex;
                        }
                    }
                }
                .el-icon{
                    font-size: 20px;
                }
            }
        }
    }
</style>
