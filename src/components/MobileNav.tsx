"use client";

import { Sheet, SheetContent, SheetTrigger, SheetClose, SheetTitle } from "./ui/sheet";
import { usePathname } from "next/navigation";
import Link from "next/link";
import { CiMenuFries } from "react-icons/ci";

const links = [
  { name: "home", path: "/" },
  { name: "projects", path: "/projects" },
  { name: "blogs", path: "/blogs" },
  { name: "contact", path: "/contact" },
];

const MobileNav = () => {
  const pathname = usePathname();

  return (
    <Sheet>
      <SheetTrigger className="flex justify-center items-center">
        <CiMenuFries className="text-[32px]" />
      </SheetTrigger>
      <SheetContent className="flex flex-col" aria-describedby={undefined}>
        <SheetTitle className="sr-only">Navigation Menu</SheetTitle> 
        {/* sr-only = screen readers only */}
        
        <div className="mt-5 mb-10 text-center text-2xl">
            uttkarzsh<span className="text-accent">.</span>
        </div>
        <nav className="flex flex-col justify-center items-center gap-8">
            {links.map((link, index) => (
            <SheetClose asChild key={index}>
                <Link
                href={link.path}
                className={`${
                    link.path === pathname &&
                    "text-accent border-b-2 border-accent"
                } font-medium hover:text-accent`}
                >
                {link.name}
                </Link>
            </SheetClose>
            ))}
        </nav>
        </SheetContent>
    </Sheet>
  );
};

export default MobileNav;