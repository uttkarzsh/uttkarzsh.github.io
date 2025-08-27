"use client";

import { Sheet, SheetContent, SheetTrigger } from "./ui/sheet";
import { usePathname } from "next/navigation";
import Link from "next/link";
import { CiMenuFries } from "react-icons/ci";

const links = [
    {
        name: "home",
        path: "/"
    },
    {
        name: "projects",
        path: "/projects"
    },
    {
        name: "blogs",
        path: "/blogs"
    },
    {
        name: "contact",
        path: "/contact"
    }
]

const MobileNav = () => {
    const pathname = usePathname();
  return (
    <Sheet>
        <SheetTrigger className="flex justify-center items-center">
            <CiMenuFries className="text-[32px]" />
        </SheetTrigger>
        <SheetContent className="flex flex-col">
            <div className="mt-5 mb-10 text-center text-2xl">
                uttkarzsh.
            </div>
            <nav className="flex flex-col justify-center items-center gap-8">
                {links.map((link, index) => {
                return <Link href={link.path} key={index} className={`${link.path === pathname && "text-accent border-b-2 border-accent"} font-medium hover:text-accent`}> {link.name} </Link>
            })}
            </nav>
        </SheetContent>
    </Sheet>
  )
}

export default MobileNav