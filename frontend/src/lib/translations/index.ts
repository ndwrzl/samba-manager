import i18n from 'sveltekit-i18n';
import lang from './lang.json';

/** @type {import('sveltekit-i18n').Config} */
const config = {
	translations: {
		en: { lang },
		es: { lang },
		eu: { lang }
	},

	loaders: [
		{
			locale: 'en',
			key: 'common',
			loader: async () => (await import('./en/common.json')).default
		},
		{
			locale: 'en',
			key: 'login',
			routes: ['/login'],
			loader: async () => (await import('./en/login.json')).default
		},
		{
			locale: 'en',
			key: 'dashboard',
			routes: ['/dashboard'],
			loader: async () => (await import('./en/dashboard.json')).default
		},
		{
			locale: 'es',
			key: 'common',
			loader: async () => (await import('./es/common.json')).default
		},
		{
			locale: 'es',
			key: 'login',
			routes: ['/login'],
			loader: async () => (await import('./es/login.json')).default
		},
		{
			locale: 'es',
			key: 'dashboard',
			routes: ['/dashboard'],
			loader: async () => (await import('./es/dashboard.json')).default
		},
		{
			locale: 'eu',
			key: 'common',
			loader: async () => (await import('./eu/common.json')).default
		},
		{
			locale: 'eu',
			key: 'login',
			routes: ['/login'],
			loader: async () => (await import('./eu/login.json')).default
		},
		{
			locale: 'eu',
			key: 'dashboard',
			routes: ['/dashboard'],
			loader: async () => (await import('./eu/dashboard.json')).default
		}
	]
};

export const { t, locale, locales, loading, loadTranslations } = new i18n(config);
