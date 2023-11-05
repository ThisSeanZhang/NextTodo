<script setup lang="ts">
import { darkTheme, NConfigProvider, NNotificationProvider } from 'naive-ui';
import { zhCN, dateZhCN } from 'naive-ui'
import SiderMenu from "./views/SiderMenu.vue";

</script>

<template>
<n-config-provider :theme="darkTheme" :locale="zhCN" :date-locale="dateZhCN">
  <n-notification-provider>
    <div style="height: 100%; position: relative;">
      <n-layout has-sider  style="height: 100%;">
        
        <SiderMenu></SiderMenu>
        
        <n-layout id="drawer-target" content-style="height: 100%; position: relative">
          <n-layout-header position="absolute" style="height: 40px" content-style="height: 100%" bordered>
            // TODO: Create Header
          </n-layout-header>
          <n-layout  position="absolute" content-style="height: 100%" style="top: 40px; padding-left: 24px;">
            <router-view style="height: 100%" />
          </n-layout>
        </n-layout>
      
        <!-- <n-layout-footer
          position="absolute"
          style="height: 30px;"
          content-style="height: 100%"
        >
          <FooterBar/>
        </n-layout-footer> -->
      </n-layout>
    </div>
  </n-notification-provider>
</n-config-provider>
</template>

<style>
.n-config-provider {
  height: 100%;
  width: 100%;
}
</style>
