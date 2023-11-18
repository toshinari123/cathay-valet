function getShortName(full_name = '') {
    return `${full_name.slice(0,2)}`
}
export default function Avatar({ children, color = '' }) {
  return (
    <div className='bg-blue-500 w-[45px] h-[45px] flex items-center justify-center rounded-full' style={{backgroundColor: color}}>
      <span className='font-bold text-sm text-white'>{getShortName(children)}</span>
    </div>
  )
}
