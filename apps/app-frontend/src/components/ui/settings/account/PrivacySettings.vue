<script setup lang="ts">
import { defineMessages, Toggle, useVIntl } from '@modrinth/ui'
import { computed, ref, watch } from 'vue'

import { useAppSettings } from '@/composables/use-app-settings'
import { get, set } from '@/helpers/settings.ts'

const { formatMessage } = useVIntl()
const settings = ref(await get())
const appSettings = useAppSettings()

const checkForUpdates = computed({
	get: () => appSettings.getFeatureFlag('check_for_updates'),
	set: (value: boolean) => {
		appSettings.featureFlags.check_for_updates = value
		settings.value.feature_flags = { ...settings.value.feature_flags, check_for_updates: value }
	},
})

const messages = defineMessages({
	privacyTitle: {
		id: 'app.settings.privacy.title',
		defaultMessage: 'Privacy & Network Activity',
	},
	privacyGuarantee: {
		id: 'app.settings.privacy.guarantee',
		defaultMessage:
			'BatxRinth is private by default and entirely free of analytics, telemetry, and advertisements. No behavioral tracking data or device identifiers are collected or uploaded.',
	},
	checkForUpdatesTitle: {
		id: 'app.settings.privacy.check-for-updates.title',
		defaultMessage: 'Check for updates',
	},
	checkForUpdatesDescription: {
		id: 'app.settings.privacy.check-for-updates.description',
		defaultMessage:
			'Ask GitHub once an hour whether a newer BatxRinth release exists. Only the release lookup is sent.',
	},
	discordRichPresenceTitle: {
		id: 'app.settings.privacy.discord-rich-presence.title',
		defaultMessage: 'Discord Rich Presence',
	},
	discordRichPresenceDescription: {
		id: 'app.settings.privacy.discord-rich-presence.description',
		defaultMessage:
			'Show BatxRinth as your current activity on Discord. Requires explicit opt-in and an app restart.',
	},
})

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)
</script>

<template>
	<div>
		<h2 class="m-0 text-lg font-semibold text-contrast">
			{{ formatMessage(messages.privacyTitle) }}
		</h2>
		<p class="m-0 mt-2 text-sm text-secondary">
			{{ formatMessage(messages.privacyGuarantee) }}
		</p>
	</div>

	<div class="mt-6 rounded-lg bg-surface-elevated p-4">
		<h3 class="m-0 mb-3 text-md font-medium text-contrast">Expected Outbound Network Requests</h3>
		<ul class="m-0 flex flex-col gap-2 p-0 text-sm list-none text-secondary">
			<li>
				<strong class="text-contrast">Game Content & Metadata:</strong> User-initiated downloads
				from Modrinth API and CDN.
			</li>
			<li>
				<strong class="text-contrast">Minecraft Runtime & Assets:</strong> Game manifests, jars, and
				asset libraries from Mojang & Minecraft servers.
			</li>
			<li>
				<strong class="text-contrast">Mod Loaders & JRE:</strong> Loader manifests from Fabric,
				Forge, NeoForge, Quilt, and Azul Java JRE downloads.
			</li>
			<li>
				<strong class="text-contrast">Microsoft Authentication:</strong> Legitimate OAuth 2.0 & Xbox
				Live authentication directly with Microsoft endpoints.
			</li>
			<li>
				<strong class="text-contrast">Application Updates:</strong> An hourly lookup of the latest
				BatxRinth release on GitHub, which you can turn off below.
			</li>
			<li>
				<strong class="text-contrast">Download Attribution:</strong> Downloads carry a header
				describing the file being fetched, so creators are credited. It contains no account, device,
				or persistent identifier.
			</li>
			<li>
				<strong class="text-contrast">Offline Local Profiles:</strong> Make no authentication
				requests at all. Mojang profile, skin, and session calls are skipped entirely.
			</li>
			<li>
				<strong class="text-contrast">Ely.by Accounts:</strong> Sign in and refresh with
				authserver.ely.by only. authlib-injector is downloaded once from GitHub and checked against
				a pinned checksum. Mojang services are never contacted.
			</li>
		</ul>
	</div>

	<div class="mt-6 flex items-center justify-between gap-4">
		<div>
			<h2 class="m-0 text-lg font-semibold text-contrast">
				{{ formatMessage(messages.checkForUpdatesTitle) }}
			</h2>
			<p class="m-0 mt-1">
				{{ formatMessage(messages.checkForUpdatesDescription) }}
			</p>
		</div>
		<Toggle id="check-for-updates" v-model="checkForUpdates" />
	</div>

	<div class="mt-6 flex items-center justify-between gap-4">
		<div>
			<h2 class="m-0 text-lg font-semibold text-contrast">
				{{ formatMessage(messages.discordRichPresenceTitle) }}
			</h2>
			<p class="m-0 mt-1">
				{{ formatMessage(messages.discordRichPresenceDescription) }}
			</p>
		</div>
		<Toggle id="disable-discord-rpc" v-model="settings.discord_rpc" />
	</div>

	<div class="mt-8 rounded-lg bg-surface-elevated p-4 text-xs text-secondary border border-border">
		<strong class="text-contrast">Independent Fork Notice:</strong><br />
		BatxRinth is an independent community fork and is not affiliated with or endorsed by Modrinth,
		Rinth, Microsoft, Mojang, or Discord.
	</div>
</template>
