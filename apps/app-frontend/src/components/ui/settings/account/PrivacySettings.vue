<script setup lang="ts">
import { defineMessages, Toggle, useVIntl } from '@modrinth/ui'
import { ref, watch } from 'vue'

import { get, set } from '@/helpers/settings.ts'

const { formatMessage } = useVIntl()
const settings = ref(await get())

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
				<strong class="text-contrast">Application Updates:</strong> Configured release checks
				directly with GitHub Releases.
			</li>
		</ul>
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
