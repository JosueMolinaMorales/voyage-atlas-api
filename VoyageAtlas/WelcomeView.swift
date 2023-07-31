//
//  WelcomeView.swift
//  VoyageAtlas
//
//  Created by Josue morales on 7/29/23.
//

import SwiftUI

struct WelcomeView: View {
    var onSignInWithEmail: () -> Void
    var body: some View {
        VStack {
            // LOGO
            HeaderView()
                .offset(y: -150)

            // SLIDESHOW
            Image("Voyage")
                .resizable()
                .frame(width: 200, height: 200)
                .offset(y: -100)
            
            // Buttons
            VoyageLongButton(text: "Sign In With Google", action: {})
            VoyageLongButton(text: "Sign In With Apple", action: {})
            VoyageLongButton(text: "Sign In With Email", action: {onSignInWithEmail()})
            VoyageLongButton(text: "Register", action: {})
        }
        .frame(maxHeight: .infinity)

    }
}

struct WelcomeView_Previews: PreviewProvider {
    static var previews: some View {
        WelcomeView(onSignInWithEmail: {})
    }
}

struct VoyageColors {
    static let orange = Color("Orange")
    static let saffron = Color("Saffron")
    static let raisinBlack = Color("RaisinBlack")
    static let silver = Color("Silver")
}
