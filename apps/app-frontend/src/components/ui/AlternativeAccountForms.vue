<template>
	<div v-if="mode === 'none'" class="flex flex-col gap-2">
		<Button type="outlined" :class="secondaryButtonClass" @click="mode = 'elyby'">
			<PlusIcon />
			{{ formatMessage(messages.useElyby) }}
		</Button>
		<Button type="outlined" :class="secondaryButtonClass" @click="mode = 'offline'">
			<PlusIcon />
			{{ formatMessage(messages.useOffline) }}
		</Button>
	</div>
	<form
		v-else
		class="flex flex-col gap-2.5 p-3 bg-surface-2 rounded-xl border border-solid border-surface-5 w-full box-border"
		@submit.prevent="submit"
	>
		<div class="flex items-center justify-between">
			<span class="text-xs text-secondary font-semibold">
				{{ formatMessage(mode === 'elyby' ? messages.elybyHeading : messages.offlineHeading) }}
			</span>
			<button
				type="button"
				class="text-xs text-secondary hover:text-primary bg-transparent border-0 cursor-pointer p-0 underline"
				@click="reset"
			>
				{{ formatMessage(messages.cancel) }}
			</button>
		</div>

		<template v-if="mode === 'elyby'">
			<input
				v-model="username"
				type="text"
				autocomplete="username"
				:placeholder="formatMessage(messages.elybyUsername)"
				:class="inputClass"
			/>
			<input
				v-model="password"
				type="password"
				autocomplete="current-password"
				:placeholder="formatMessage(messages.elybyPassword)"
				:class="inputClass"
			/>
			<input
				v-model="totp"
				type="text"
				inputmode="numeric"
				autocomplete="one-time-code"
				maxlength="6"
				:placeholder="formatMessage(messages.elybyTotp)"
				:class="inputClass"
			/>
			<p class="m-0 text-xs text-secondary">{{ formatMessage(messages.elybyNote) }}</p>
		</template>
		<template v-else>
			<input
				v-model="username"
				type="text"
				:placeholder="formatMessage(messages.offlinePlaceholder)"
				maxlength="16"
				:class="inputClass"
			/>
			<p class="m-0 text-xs text-secondary">{{ formatMessage(messages.offlineNote) }}</p>
		</template>

		<Button
			type="colored"
			color="brand"
			class="w-full justify-center"
			:disabled="busy || !canSubmit"
			@click="submit"
		>
			<LogInIcon v-if="!busy" />
			<SpinnerIcon v-else class="animate-spin" />
			{{ formatMessage(mode === 'elyby' ? messages.elybySubmit : messages.offlineSubmit) }}
		</Button>
	</form>
</template>

<script setup lang="ts">
import { LogInIcon, PlusIcon, SpinnerIcon } from '@modrinth/assets'
import { Button, defineMessages, injectNotificationManager, useVIntl } from '@modrinth/ui'
import { computed, ref } from 'vue'

import { login_elyby, login_offline } from '@/helpers/auth'

const emit = defineEmits<{
	signedIn: [credentials: { profile: { id: string; name: string } }]
}>()

const { formatMessage } = useVIntl()
const { handleError } = injectNotificationManager()

const secondaryButtonClass =
	'w-full !bg-surface-2 !text-primary border border-solid border-surface-5 hover:!bg-surface-3 cursor-pointer justify-center'
const inputClass =
	'w-full box-border text-sm px-3 py-2 rounded-lg bg-bg text-primary border border-solid border-surface-5 focus:outline-none focus:border-brand'

const mode = ref<'none' | 'offline' | 'elyby'>('none')
const username = ref('')
const password = ref('')
const totp = ref('')
const busy = ref(false)

const canSubmit = computed(
	() => username.value.trim() !== '' && (mode.value !== 'elyby' || password.value !== ''),
)

function reset() {
	mode.value = 'none'
	username.value = ''
	password.value = ''
	totp.value = ''
}

async function submit() {
	if (busy.value || !canSubmit.value) return
	busy.value = true
	try {
		const credentials =
			mode.value === 'elyby'
				? await login_elyby(username.value.trim(), password.value, totp.value.trim() || null)
				: await login_offline(username.value.trim())
		if (credentials) {
			emit('signedIn', credentials)
			reset()
		}
	} catch (error) {
		handleError(error as Error)
	} finally {
		password.value = ''
		busy.value = false
	}
}

const messages = defineMessages({
	useElyby: {
		id: 'app.alternative-accounts.use-elyby',
		defaultMessage: 'Sign in with Ely.by',
	},
	useOffline: {
		id: 'app.alternative-accounts.use-offline',
		defaultMessage: 'Use an offline profile',
	},
	cancel: {
		id: 'app.alternative-accounts.cancel',
		defaultMessage: 'Cancel',
	},
	elybyHeading: {
		id: 'app.alternative-accounts.elyby.heading',
		defaultMessage: 'Ely.by account',
	},
	elybyUsername: {
		id: 'app.alternative-accounts.elyby.username',
		defaultMessage: 'Username or email',
	},
	elybyPassword: {
		id: 'app.alternative-accounts.elyby.password',
		defaultMessage: 'Password',
	},
	elybyTotp: {
		id: 'app.alternative-accounts.elyby.totp',
		defaultMessage: '2FA code (only if enabled)',
	},
	elybyNote: {
		id: 'app.alternative-accounts.elyby.note',
		defaultMessage:
			'Signs in through authserver.ely.by. Your password is sent only to Ely.by and is not stored. Ely.by skins and Ely.by-enabled servers work in game.',
	},
	elybySubmit: {
		id: 'app.alternative-accounts.elyby.submit',
		defaultMessage: 'Sign in with Ely.by',
	},
	offlineHeading: {
		id: 'app.alternative-accounts.offline.heading',
		defaultMessage: 'Offline username',
	},
	offlinePlaceholder: {
		id: 'app.alternative-accounts.offline.placeholder',
		defaultMessage: 'e.g. Steve',
	},
	offlineNote: {
		id: 'app.alternative-accounts.offline.note',
		defaultMessage:
			'Local testing profile only. It does not prove game ownership, and cannot join online-mode servers or use Mojang skins and capes.',
	},
	offlineSubmit: {
		id: 'app.alternative-accounts.offline.submit',
		defaultMessage: 'Sign in offline',
	},
})
</script>
