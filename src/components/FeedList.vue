<script setup lang="ts">
import { ref } from 'vue'
import type { Feed } from '@/types'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { useI18n } from '@/composables/useI18n'
import { validateFeedUrl } from '@/utils/validators'

interface Props {
  feeds: Feed[]
  loading: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
}>()

const { addFeed, deleteFeed, refreshFeed, refreshAllFeeds } = useFeeds()
const { showError } = useToast()
const { t } = useI18n()

const showAddDialog = ref(false)
const newFeedUrl = ref('')
const newFeedCategory = ref('')
const adding = ref(false)
const selectedFeedId = ref<number | undefined>(undefined)

const handleSelectFeed = (feedId: number) => {
  selectedFeedId.value = feedId
  emit('feed-selected', feedId)
}

const openAddDialog = () => {
  newFeedUrl.value = ''
  newFeedCategory.value = ''
  showAddDialog.value = true
}

const handleAddFeed = async () => {
  const validation = validateFeedUrl(newFeedUrl.value)
  if (!validation.valid) {
    showError(validation.error)
    return
  }
  adding.value = true
  const success = await addFeed(newFeedUrl.value, newFeedCategory.value || undefined)
  adding.value = false
  if (success) showAddDialog.value = false
}

const handleDeleteFeed = async (feedId: number, feedTitle: string) => {
  const confirmed = confirm(`确定要删除订阅源 "${feedTitle}" 吗？`)
  if (!confirmed) return
  const success = await deleteFeed(feedId)
  if (success && selectedFeedId.value === feedId) {
    selectedFeedId.value = undefined
  }
}

const handleRefreshFeed = async (feedId: number) => {
  await refreshFeed(feedId)
}

const handleRefreshAll = async () => {
  await refreshAllFeeds()
}
</script>

<template>
  <div class="pa-2">
    <!-- Toolbar -->
    <div class="d-flex ga-2 mb-3 px-2">
      <v-btn
        :disabled="loading"
        color="primary"
        size="small"
        class="flex-1-1"
        @click="openAddDialog"
      >
        <v-icon start size="16">mdi-plus</v-icon>
        {{ $t('feeds.addFeed') }}
      </v-btn>
      <v-btn
        :disabled="loading || feeds.length === 0"
        color="primary"
        variant="outlined"
        size="small"
        class="flex-1-1"
        @click="handleRefreshAll"
      >
        <v-icon start size="16">mdi-refresh</v-icon>
        {{ $t('refresh.refreshAll') }}
      </v-btn>
    </div>

    <!-- Feed List -->
    <div v-if="feeds.length === 0 && !loading" class="text-center pa-6 text-body-2 text-medium-emphasis">
      <p>{{ $t('feeds.noFeeds') }}</p>
      <p class="mt-2 text-caption">{{ $t('feeds.noFeedsDesc') }}</p>
    </div>

    <v-list v-else density="compact" class="bg-transparent" :lines="false">
      <v-list-item
        v-for="feed in feeds"
        :key="feed.id"
        :active="selectedFeedId === feed.id"
        color="primary"
        rounded
        class="mb-1"
        @click="handleSelectFeed(feed.id)"
      >
        <template v-slot:prepend>
          <v-avatar size="32" rounded color="primary" class="mr-2">
            <span class="text-caption font-weight-bold text-white">{{ feed.title.charAt(0).toUpperCase() }}</span>
          </v-avatar>
        </template>

        <v-list-item-title class="text-body-2 font-weight-medium">{{ feed.title }}</v-list-item-title>
        <v-list-item-subtitle v-if="feed.category" class="text-caption">
          {{ feed.category }}
        </v-list-item-subtitle>

        <template v-slot:append>
          <div class="d-flex ga-0">
            <v-btn icon variant="text" size="x-small" :title="$t('feeds.editFeed')" @click.stop="handleRefreshFeed(feed.id)">
              <v-icon size="14">mdi-refresh</v-icon>
            </v-btn>
            <v-btn icon variant="text" size="x-small" :title="$t('feeds.deleteFeed')" @click.stop="handleDeleteFeed(feed.id, feed.title)">
              <v-icon size="14">mdi-close</v-icon>
            </v-btn>
          </div>
        </template>
      </v-list-item>
    </v-list>

    <!-- Add Feed Dialog -->
    <v-dialog v-model="showAddDialog" max-width="420">
      <v-card>
        <v-card-title class="text-body-1 font-weight-semibold">{{ $t('feeds.addFeed') }}</v-card-title>

        <v-card-text>
          <v-text-field
            v-model="newFeedUrl"
            :label="$t('feeds.feedUrl')"
            placeholder="https://example.com/feed.xml"
            density="compact"
            variant="outlined"
            class="mb-3"
            @keydown.enter="handleAddFeed"
          />
          <v-text-field
            v-model="newFeedCategory"
            :label="$t('feeds.feedCategory')"
            :placeholder="$t('common.edit')"
            density="compact"
            variant="outlined"
            @keydown.enter="handleAddFeed"
          />
        </v-card-text>

        <v-divider />

        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showAddDialog = false">{{ $t('common.cancel') }}</v-btn>
          <v-btn
            color="primary"
            :loading="adding"
            :disabled="!newFeedUrl"
            @click="handleAddFeed"
          >
            {{ $t('common.save') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.flex-1-1 {
  flex: 1 1 0;
}
</style>
