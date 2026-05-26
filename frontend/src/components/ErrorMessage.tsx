import { FC } from 'react'
import { AlertTriangleIcon } from './Icons'
import '../styles/components.css'

interface ErrorMessageProps {
  message: string
}

const ErrorMessage: FC<ErrorMessageProps> = ({ message }) => {
  return (
    <div className="error-message">
      <AlertTriangleIcon className="error-icon" size={20} />
      <span className="error-text">{message}</span>
    </div>
  )
}

export default ErrorMessage
