import React from 'react';

interface FormErrorProps {
  message?: string;
}

const FormError: React.FC<FormErrorProps> = ({ message }) => {
  if (!message) return null;

  return <p className="text-red-600 text-sm mt-1">{message}</p>;
};

export default FormError;
