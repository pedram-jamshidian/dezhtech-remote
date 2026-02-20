import 'package:flutter/material.dart';
import 'package:flutter_hbb/common.dart';
import 'package:flutter_hbb/consts.dart';
import 'package:flutter_hbb/desktop/pages/desktop_home_page.dart';
import 'package:flutter_hbb/desktop/pages/desktop_setting_page.dart';
import 'package:flutter_hbb/desktop/widgets/tabbar_widget.dart';
import 'package:flutter_hbb/models/platform_model.dart';
import 'package:get/get.dart';
import 'package:window_manager/window_manager.dart';

// ===========================================================================
// DezhTech Remote - Desktop Tab Page
// ===========================================================================

class DesktopTabPage extends StatefulWidget {
  const DesktopTabPage({Key? key}) : super(key: key);

  @override
  State<DesktopTabPage> createState() => _DesktopTabPageState();

  static void onAddSetting({int initialPage = 0}) {
    try {
      DesktopTabController tabController = Get.find<DesktopTabController>();
      final index = tabController.state.value.tabs
          .indexWhere((tab) => tab.key == kTabLabelSettingPage);
      if (index >= 0) {
        tabController.jumpTo(index);
      } else {
        tabController.add(TabInfo(
            key: kTabLabelSettingPage,
            label: kTabLabelSettingPage,
            closable: true,
            page: DesktopSettingPage(initialTabKey: SettingsTabKey.values[initialPage])));
      }
    } catch (e) {
      debugPrintStack(label: '$e');
    }
  }
}

class _DesktopTabPageState extends State<DesktopTabPage>
    with TickerProviderStateMixin, WindowListener {
  final tabController = Get.put(DesktopTabController());

  @override
  void initState() {
    super.initState();
    tabController.add(TabInfo(
        key: kTabLabelHomePage,
        label: kTabLabelHomePage,
        closable: false,
        page: const DesktopHomePage()));
    windowManager.addListener(this);
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  @override
  void onWindowFocus() {
    // Do something when window gets focus
  }

  @override
  Widget build(BuildContext context) {
    // DezhTech Remote - Main Desktop Tab Container
    return DragToResizeArea(
      child: Container(
        decoration: BoxDecoration(
          border: Border.all(color: MyTheme.color(context).border ?? Colors.transparent),
        ),
        child: Scaffold(
          backgroundColor: Theme.of(context).colorScheme.background,
          body: Column(
            children: [
              // Tab Bar - DezhTech Remote Style
              DesktopTitleBar(
                child: TabBar(
                  controller: tabController.tabController,
                  tabs: tabController.state.value.tabs.map((tab) {
                    return Tab(
                      child: Row(
                        mainAxisSize: MainAxisSize.min,
                        children: [
                          Text(translate(tab.label)),
                          if (tab.closable)
                            InkWell(
                              onTap: () => tabController.remove(tabController.state.value.tabs.indexOf(tab)),
                              child: const Icon(Icons.close, size: 16),
                            ).marginOnly(left: 8),
                        ],
                      ),
                    );
                  }).toList(),
                ),
              ),
              // Tab Content
              Expanded(
                child: Obx(
                  () => IndexedStack(
                    index: tabController.state.value.selected,
                    children: tabController.state.value.tabs.map((tab) => tab.page).toList(),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  @override
  void onWindowClose() async {
    await windowManager.hide();
    super.onWindowClose();
  }
}
