import React from 'react';
import type { UseFormRegister, FieldErrors } from 'react-hook-form';

interface InputProps {
  label: string;
  name: string;
  type?: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  register: UseFormRegister<any>;
  errors?: FieldErrors;
  required?: boolean;
  placeholder?: string;
}

const Input: React.FC<InputProps> = ({
  label,
  name,
  type = 'text',
  register,
  errors,
  required = false,
  placeholder,
}) => {
  const error = errors ? errors[name] : undefined;

  return (
    <div className="mb-4">
      <label htmlFor={name} className="block text-sm font-medium text-gray-700 dark:text-gray-300">
        {label}{required && ' *'}
      </label>
      <input
        id={name}
        type={type}
        placeholder={placeholder}
        {...register(name, { required })}
        className={`mt-1 w-full px-4 py-2 border rounded-lg bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500 ${error ? 'border-red-500' : 'border-gray-300'
          }`}
      />
      {error && (
        <p className="text-red-600 text-sm mt-1">
          {error.type === 'required' ? `${label} is required` : 'Invalid input'}
        </p>
      )}
    </div>
  );
};

export default Input;
