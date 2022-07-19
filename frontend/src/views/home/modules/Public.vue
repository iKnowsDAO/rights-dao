<template>
    <div class="home-public-container">
        <div class="container">
            <div style="max-width: 800px;line-height: 1.8;">
                <h2 class="hello">{{t('post.public.hello')}}✍</h2>
                <span>{{t('post.public.text')}}</span>
            </div>
        </div>
    </div>
</template>
<script lang="ts" setup>
    import {t} from '@/locale';
</script>
<style lang="scss">
    .home-public-container {
        border-bottom: 1px solid rgb(228, 228, 231);
        background-color: rgb(255,255,255);
        .container{
            padding-top: 80px;
            padding-bottom: 80px;
            .hello{
                font-weight: 800;
            }
        }
    }
</style>
