import i18n from 'sveltekit-i18n';
import lang from './lang.json';
import { browser } from '$app/environment';
import Cookies from 'js-cookie';

function storage_or_default() {
	// return localStorage.getItem('language') || 'en';
	return Cookies.get('lang') || 'en';
}

/** @type {import('sveltekit-i18n').Config} */
const config = {
	translations: {
		eu: { lang },
		en: { lang },
		es: { lang }
	},
	fallbackLocale: 'en',
	loaders: [
		{
			locale: 'eu',
			key: 'l',
			loader: async () => (await import('./eu.json')).default
		},
		{
			locale: 'en',
			key: 'l',
			loader: async () => (await import('./en.json')).default
		},
		{
			locale: 'es',
			key: 'l',
			loader: async () => (await import('./es.json')).default
		}
	],
	'log.level': 'error'
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(config);
loadTranslations(storage_or_default(), '/');

if (browser) {
	locale.subscribe((lang) => {
		// localStorage.setItem('language', lang);
		Cookies.set('lang', lang, { expires: 365 });
	});
}
