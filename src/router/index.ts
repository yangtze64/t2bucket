import { createRouter, createWebHistory } from "vue-router";
import HomePage from "../pages/HomePage.vue";
import ConnectionForm from "../pages/ConnectionForm.vue";
import BucketBrowser from "../pages/BucketBrowser.vue";
import ObjectPreview from "../pages/ObjectPreview.vue";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", name: "home", component: HomePage },
    { path: "/connection/new", name: "new-connection", component: ConnectionForm },
    { path: "/connection/:id/edit", name: "edit-connection", component: ConnectionForm },
    { path: "/browse/:connectionId", name: "browse-buckets", component: BucketBrowser },
    { path: "/browse/:connectionId/:bucket", name: "browse-objects", component: BucketBrowser },
    { path: "/preview/:connectionId/:bucket/:key(.*)", name: "preview", component: ObjectPreview },
  ],
});

export default router;
