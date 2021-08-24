{-# LANGUAGE CPP #-}
{-# LANGUAGE NoRebindableSyntax #-}
{-# OPTIONS_GHC -fno-warn-missing-import-lists #-}
module Paths_fizzbuzz (
    version,
    getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir,
    getDataFileName, getSysconfDir
  ) where

import qualified Control.Exception as Exception
import Data.Version (Version(..))
import System.Environment (getEnv)
import Prelude

#if defined(VERSION_base)

#if MIN_VERSION_base(4,0,0)
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#else
catchIO :: IO a -> (Exception.Exception -> IO a) -> IO a
#endif

#else
catchIO :: IO a -> (Exception.IOException -> IO a) -> IO a
#endif
catchIO = Exception.catch

version :: Version
version = Version [0,1,0,0] []
bindir, libdir, dynlibdir, datadir, libexecdir, sysconfdir :: FilePath

bindir     = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/bin"
libdir     = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/lib/x86_64-osx-ghc-8.10.6/fizzbuzz-0.1.0.0-DTKiy7jaZ811xUNUdbknTQ-fizzbuzz"
dynlibdir  = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/lib/x86_64-osx-ghc-8.10.6"
datadir    = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/share/x86_64-osx-ghc-8.10.6/fizzbuzz-0.1.0.0"
libexecdir = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/libexec/x86_64-osx-ghc-8.10.6/fizzbuzz-0.1.0.0"
sysconfdir = "/Users/dentaku/dev/notes3/fizzbuzz/.stack-work/install/x86_64-osx/7c9341b7ade87284172d0b9fd0909107321c222282622c210e341ce6d523eb7c/8.10.6/etc"

getBinDir, getLibDir, getDynLibDir, getDataDir, getLibexecDir, getSysconfDir :: IO FilePath
getBinDir = catchIO (getEnv "fizzbuzz_bindir") (\_ -> return bindir)
getLibDir = catchIO (getEnv "fizzbuzz_libdir") (\_ -> return libdir)
getDynLibDir = catchIO (getEnv "fizzbuzz_dynlibdir") (\_ -> return dynlibdir)
getDataDir = catchIO (getEnv "fizzbuzz_datadir") (\_ -> return datadir)
getLibexecDir = catchIO (getEnv "fizzbuzz_libexecdir") (\_ -> return libexecdir)
getSysconfDir = catchIO (getEnv "fizzbuzz_sysconfdir") (\_ -> return sysconfdir)

getDataFileName :: FilePath -> IO FilePath
getDataFileName name = do
  dir <- getDataDir
  return (dir ++ "/" ++ name)
