import type { Load } from '@sveltejs/kit';

export const prerender = true;
export const trailingSlash = 'always';

import { loadTranslations } from '$lib/translations';
export const load: Load = async ({ url }) => {
	const { pathname } = url;

	const initLocale = 'es';

	await loadTranslations(initLocale, pathname);

	return {};
};
