import { useForm } from 'react-hook-form';
import Input from '../components/form/Input';
import { useTranslation } from 'react-i18next';
import LanguageSwitcher from '../components/LanguageSwitcher';

interface LoginFormInputs {
  email: string;
  password: string;
}

const Login = () => {
  const { t } = useTranslation();
  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormInputs>();

  const onSubmit = (data: LoginFormInputs) => {
    console.log(data);
  };

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 p-4">
      <LanguageSwitcher />
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="bg-white dark:bg-gray-800 p-8 rounded-2xl shadow-md w-full max-w-md mt-6"
      >
        <h2 className="text-2xl font-bold mb-6 text-center text-gray-900 dark:text-white">
          {t('login.title')}
        </h2>
        <Input
          label={t('login.email')}
          name="email"
          type="email"
          register={register}
          errors={errors}
          required
          placeholder={t('login.email')}
        />
        <Input
          label={t('login.password')}
          name="password"
          type="password"
          register={register}
          errors={errors}
          required
          placeholder={t('login.password')}
        />
        <button
          type="submit"
          className="w-full py-2 px-4 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition"
        >
          {t('login.submit')}
        </button>
      </form>
    </div>
  );
};

export default Login;
