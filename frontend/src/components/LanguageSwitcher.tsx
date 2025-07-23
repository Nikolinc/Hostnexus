import { useTranslation } from 'react-i18next';

const LanguageSwitcher = () => {
  const { i18n } = useTranslation();

  const changeLanguage = (lng: string) => {
    i18n.changeLanguage(lng);
  };

  return (
    <div className="flex gap-2">
      <button
        onClick={() => changeLanguage('en')}
        className="px-3 py-1 rounded border hover:bg-gray-200"
      >
        EN
      </button>
      <button
        onClick={() => changeLanguage('ru')}
        className="px-3 py-1 rounded border hover:bg-gray-200"
      >
        RU
      </button>
    </div>
  );
};

export default LanguageSwitcher;

