import Link from "next/link"
import {FaGithub, FaLinkedin} from "react-icons/fa"
import {RiTwitterXFill} from "react-icons/ri"


const socials = [
    {icon: <FaGithub/>, path:"https://github.com/uttkarzsh"},
    {icon: <FaLinkedin/>, path:"https://www.linkedin.com/in/uttkarsh-pawan/"},
    {icon: <RiTwitterXFill/>, path:"https://x.com/0xuttkarzsh"},
]

interface SocialsProps {
  containerStyles?: string;
  iconStyles?: string;
}

const Socials = ({containerStyles, iconStyles}: SocialsProps) => {
  return (
    <div className={containerStyles}>
        {socials.map((item, index) =>{
            return <Link key={index} href={item.path} className={iconStyles}>{item.icon}</Link>
        })}
    </div>
  )
}

export default Socials