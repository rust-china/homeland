import { createRouter, createWebHistory, createMemoryHistory } from 'vue-router'

const router = createRouter({
  history: import.meta.env.SSR ? createMemoryHistory(import.meta.env.BASE_URL) : createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      redirect: { path: '/posts' }
    },
    {
      path: '/auth',
      name: 'auth',
      component: () => import('@/views/auth/index.vue'),
      children: [
        { path: 'login', name: 'auth/login', component: () => import('@/views/auth/login/index.vue') }
      ]
    },
    {
      path: '/posts',
      name: 'posts',
      component: () => import('@/views/posts/index.vue'),
      redirect: { name: 'posts/default'  },
      children: [
        { path: 'default', name: 'posts/default', component: () => import('@/views/posts/list/index.vue') },
        { path: 'last', name: 'posts/last', component: () => import('@/views/posts/list/index.vue') },
        { path: 'excellent', name: 'posts/excellent', component: () => import('@/views/posts/list/index.vue') },
        { path: 'popular', name: 'posts/popular', component: () => import('@/views/posts/list/index.vue') },
        { path: 'lastComment', name: 'posts/lastComment', component: () => import('@/views/posts/list/index.vue') },
        { path: 'new', name: 'posts/new', component: () => import('@/views/posts/form/index.vue') },
        { path: 'edit/:uuid', name: 'posts/edit', component: () => import('@/views/posts/form/index.vue') },
        { path: ':uuid', name: 'posts/show', component: () => import('@/views/posts/show/index.vue') }
      ]
    },
    {
      path: '/jobs',
      component: () => import('@/views/jobs/index.vue'),
      children: [
        { path: '', name: 'jobs', component: () => import('@/views/jobs/list/index.vue') }
      ]
    },
    {
      path: '/wiki',
      component: () => import('@/views/wiki/index.vue'),
      children: [
        { path: '', name: 'wiki', component: () => import('@/views/wiki/list/index.vue') }
      ]
    },
    {
      path: '/sites',
      component: () => import('@/views/sites/index.vue'),
      children: [
        { path: '', name: 'sites', component: () => import('@/views/sites/list/index.vue') }
      ]
    },
    {
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('@/views/about/index.vue')
    }
  ]
})

export default router
