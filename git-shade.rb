class git-shade < Formula
  desc ""
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.0"
  license ""

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.0/git-shade-v1.0.0-x86_64-apple-darwin.tar.gz"
      sha256 "59801f35552fe7a7398c1fa04758b136ce65512281a12fff876fc61f93fb3775"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.0/git-shade-v1.0.0-aarch64-apple-darwin.tar.gz"
      sha256 "4d02a64d2f6b491c310539d4d6d7df21f14d20be511f4f5b83811dc2637270a3"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
