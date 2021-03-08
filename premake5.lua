workspace "Jobsjoggt"
   configurations { "Debug", "Release" }

project "Jobsjoggt"
   kind "ConsoleApp"
   language "C++"
   cppdialect "C++17"

   targetdir "bin/%{cfg.buildcfg}"
   objdir "bin-int/%{cfg.buildcfg}"

   files { "**.h", "**.c" }

   filter "configurations:Debug"
      --defines { "DEBUG" }
      symbols "On"

   filter "configurations:Release"
      --defines { "NDEBUG" }
      optimize "On"
