<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Category, Feed } from '@/types'
import { useFeeds } from '@/composables/useFeeds'
import { useToast } from '@/composables/useToast'
import { useI18n } from '@/composables/useI18n'
import { validateFeedUrl } from '@/utils/validators'

interface Props {
  categories: Category[]
  feeds: Feed[]
  loading: boolean
  selectedFeedId?: number
}

const props = defineProps<Props>()

const emit = defineEmits<{
  (e: 'feed-selected', feedId: number): void
  (e: 'feed-delete', feedId: number): void
  (e: 'feed-refresh', feedId: number): void
  (e: 'feed-updated'): void
}>()

const { addFeed, updateFeed } = useFeeds()
const { showError } = useToast()
const { t } = useI18n()

const showAddDialog = ref(false)
const newFeedUrl = ref('')
const newFeedCategory = ref('')
const adding = ref(false)
const showEditDialog = ref(false)
const editingFeed = ref<Feed | null>(null)
const editTitle = ref('')
const editUrl = ref('')
const editCategory = ref('')
const saving = ref(false)

const categoryNames = computed(() => props.categories.map(c => c.name))

const getFeedsByCategory = (categoryName: string | null): Feed[] => {
  if (categoryName === null) {
    return props.feeds.filter(feed => !feed.category)
  }
  return props.feeds.filter(feed => feed.category?.trim() === categoryName)
}

const handleSelectFeed = (feedId: number) => {
  emit('feed-selected', feedId)
}

const handleDeleteFeed = (feedId: number, feedTitle: string) => {
  const confirmed = confirm(`确定要删除 "${feedTitle}" 吗？`)
  if (confirmed) emit('feed-delete', feedId)
}

const handleRefreshFeed = (feedId: number) => {
  emit('feed-refresh', feedId)
}

const openAddDialog = () => {
  newFeedUrl.value = ''
  newFeedCategory.value = ''
  showAddDialog.value = true
}

const handleAddFeed = async () => {
  const validation = validateFeedUrl(newFeedUrl.value)
  if (!validation.valid) {
    showError(validation?.error)
    return
  }
  adding.value = true
  const success = await addFeed(newFeedUrl.value, newFeedCategory.value.trim() || undefined)
  adding.value = false
  if (success) {
    showAddDialog.value = false
    emit('feed-updated')
  }
}

const openEditDialog = (feed: Feed) => {
  editingFeed.value = feed
  editTitle.value = feed.title
  editUrl.value = feed.url
  editCategory.value = feed.category || ''
  showEditDialog.value = true
}

const handleEditFeed = async () => {
  if (!editingFeed.value) return

  // Validate URL if changed
  if (editUrl.value !== editingFeed.value.url) {
    const validation = validateFeedUrl(editUrl.value)
    if (!validation.valid) {
      showError(validation.error)
      return
    }
  }

  saving.value = true
  const updates: { title?: string; url?: string; category?: string } = {}
  if (editTitle.value !== editingFeed.value.title) updates.title = editTitle.value
  if (editUrl.value !== editingFeed.value.url) updates.url = editUrl.value
  if (editCategory.value.trim() !== (editingFeed.value.category || '')) {
    const trimmed = editCategory.value.trim()
    updates.category = trimmed || undefined
  }

  if (Object.keys(updates).length === 0) {
    showEditDialog.value = false
    saving.value = false
    return
  }

  const success = await updateFeed(editingFeed.value.id, updates)
  saving.value = false
  if (success) {
    showEditDialog.value = false
    emit('feed-updated')
  }
}
</script>

<template>
  <div class="px-3">
    <!-- Add Feed Button -->
    <v-btn
      :disabled="loading"
      variant="text"
      block
      size="large"
      class="mb-1"
      @click="openAddDialog"
    >
      <v-icon start size="16">mdi-plus</v-icon>
      {{ $t('feeds.addFeed') }}
    </v-btn>

    <!-- Empty state -->
    <div
      v-if="categories.length === 0 && !loading"
      class="py-8 text-center text-body-2 text-medium-emphasis"
    >
      {{ $t('categories.allCategories') }}
    </div>

    <!-- Categories -->
    <v-list v-else density="compact" class="bg-transparent" :lines="false">
      <v-list-group v-for="category in categories" :key="category.name" :value="category.name">
        <template v-slot:activator="{ props: listProps }">
          <v-list-item v-bind="listProps" class="cat-header rounded">
            <v-list-item-title class="text-body-2 font-weight-medium text-uppercase tracking-wide">
              {{ category.name }}
            </v-list-item-title>
            <template v-slot:append>
              <v-chip
                v-if="category.unread_count > 0"
                size="x-small"
                :color="isWarmInk ? 'primary' : 'primary'"
                variant="tonal"
                class="ml-1"
              >
                {{ category.unread_count }}
              </v-chip>
            </template>
          </v-list-item>
        </template>

        <!-- Feeds under category -->
        <v-list-item
          v-for="feed in getFeedsByCategory(category.name)"
          :key="feed.id"
          :active="selectedFeedId === feed.id"
          color="primary"
          rounded
          class="feed-item rounded"
          @click="handleSelectFeed(feed.id)"
        >
          <template v-slot:prepend>
            <v-avatar
              size="24"
              rounded
              :color="selectedFeedId === feed.id ? 'primary' : 'surface-variant'"
              class="mr-1"
            >
              <span class="text-caption font-weight-bold">{{
                feed.title.charAt(0).toUpperCase()
              }}</span>
            </v-avatar>
          </template>

          <v-list-item-title class="text-body-2">{{ feed.title }}</v-list-item-title>

          <template v-slot:append>
            <div class="d-flex ga-0">
              <v-btn
                icon
                variant="text"
                size="x-small"
                :title="$t('feeds.editFeed')"
                @click.stop="openEditDialog(feed)"
              >
                <v-icon size="14">mdi-pencil</v-icon>
              </v-btn>
              <v-btn
                icon
                variant="text"
                size="x-small"
                title="刷新"
                @click.stop="handleRefreshFeed(feed.id)"
              >
                <v-icon size="14">mdi-refresh</v-icon>
              </v-btn>
              <v-btn
                icon
                variant="text"
                size="x-small"
                title="删除"
                @click.stop="handleDeleteFeed(feed.id, feed.title)"
              >
                <v-icon size="14">mdi-close</v-icon>
              </v-btn>
            </div>
          </template>
        </v-list-item>

        <div
          v-if="getFeedsByCategory(category.name).length === 0"
          class="px-3 py-2 text-caption text-medium-emphasis"
        >
          {{ $t('feeds.noFeeds') }}
        </div>
      </v-list-group>
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
            hint="必填"
            persistent-hint
            @keydown.enter="handleAddFeed"
          />
          <v-combobox
            v-model="newFeedCategory"
            :items="categoryNames"
            :label="$t('feeds.feedCategory')"
            :placeholder="$t('common.edit')"
            density="compact"
            variant="outlined"
            clearable
            @keydown.enter="handleAddFeed"
          />
        </v-card-text>

        <v-divider />

        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showAddDialog = false">{{ $t('common.cancel') }}</v-btn>
          <v-btn color="primary" :loading="adding" :disabled="!newFeedUrl" @click="handleAddFeed">
            {{ $t('common.save') }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Edit Feed Dialog -->
    <v-dialog v-model="showEditDialog" max-width="420">
      <v-card>
        <v-card-title class="text-body-1 font-weight-semibold">{{ $t('feeds.editFeed') }}</v-card-title>

        <v-card-text>
          <v-text-field
            v-model="editTitle"
            :label="$t('feeds.feedTitle')"
            density="compact"
            variant="outlined"
            class="mb-3"
            @keydown.enter="handleEditFeed"
          />
          <v-text-field
            v-model="editUrl"
            :label="$t('feeds.feedUrl')"
            density="compact"
            variant="outlined"
            class="mb-3"
            @keydown.enter="handleEditFeed"
          />
          <v-combobox
            v-model="editCategory"
            :items="categoryNames"
            :label="$t('feeds.feedCategory')"
            :placeholder="$t('common.edit')"
            density="compact"
            variant="outlined"
            clearable
            @keydown.enter="handleEditFeed"
          />
        </v-card-text>

        <v-divider />

        <v-card-actions>
          <v-spacer />
          <v-btn variant="text" @click="showEditDialog = false">{{ $t('common.cancel') }}</v-btn>
          <v-btn color="primary" :loading="saving" @click="handleEditFeed"> {{ $t('common.save') }} </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<style scoped>
.feed-item :deep(.v-list-item__append) {
  opacity: 0;
  transition: opacity 100ms;
}
.feed-item:hover :deep(.v-list-item__append) {
  opacity: 1;
}
</style>
